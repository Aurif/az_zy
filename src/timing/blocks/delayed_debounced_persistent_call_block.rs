use std::fs;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chain_drive::{ChainBBack, ChainBFront, ChainBlock, ChainDrive, ChainJumper, ChainJumperCore, ChainJumpResult, ChainPayload, InitPayload};
use tokio::task::JoinHandle;
use crate::timing::timing_service::{TimingService};

impl TimingService {
    pub fn delayed_debounced_persistent_call_block<C: ChainPayload + Send + Sync + 'static, P: ChainPayload + 'static>(&self, timeout: Duration, func: fn() -> P) -> DelayedDebouncedPersistentCallBlock<C, P> {
        DelayedDebouncedPersistentCallBlock {
            timeout,
            phantom_data: PhantomData,
            handle: None,
            func,
        }
    }
}
// WARNING: not tested!
pub struct DelayedDebouncedPersistentCallBlock<C: ChainPayload + Send + Sync, P: ChainPayload + 'static> {
    timeout: Duration,
    phantom_data: PhantomData<C>,
    handle: Option<JoinHandle<()>>,
    func: fn() -> P,
}
impl<C: ChainPayload + Send + Sync + 'static, P: ChainPayload + 'static> DelayedDebouncedPersistentCallBlock<C, P> {
    fn new_handle(&mut self, duration: Duration, jump: ChainJumperCore) {
        if let Some(handle) = &self.handle {
            handle.abort();
        }
        let func = self.func.clone();
        let handle = tokio::spawn(async move {
            tokio::time::sleep(duration).await;
            fs::remove_file("./delayed_call_time.txt").unwrap();
            jump.emit(func());
        });
        self.handle = Some(handle);
    }
}

impl<C: ChainPayload + Send + Sync + 'static, P: ChainPayload + 'static> ChainBlock<InitPayload, ChainBBack> for DelayedDebouncedPersistentCallBlock<C, P> {
    fn run(&mut self, payload: InitPayload, jump: ChainJumper<InitPayload>) -> ChainJumpResult {
        if let Ok(cache) = fs::read_to_string("./delayed_call_time.txt") {
            let stored_target_timestamp: u64 = cache.trim().parse().unwrap();
            let timestamp_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            if stored_target_timestamp > timestamp_now {
                self.new_handle(Duration::from_secs(1), jump.get_core())
            } else {
                self.new_handle(Duration::from_secs(timestamp_now - stored_target_timestamp), jump.get_core())
            }
        }
        jump.next(payload)
    }
}

impl<C: ChainPayload + Send + Sync + 'static, P: ChainPayload + 'static> ChainBlock<C, ChainBFront> for DelayedDebouncedPersistentCallBlock<C, P> {
    fn run(&mut self, payload: C, jump: ChainJumper<C>) -> ChainJumpResult {
        self.new_handle(self.timeout.clone(), jump.get_core());

        let target_timestamp = (SystemTime::now() + self.timeout).duration_since(UNIX_EPOCH).unwrap().as_secs();
        fs::write("./delayed_call_time.txt", format!("{target_timestamp}")).expect("Unable to write file");

        jump.next(payload)
    }
}

impl<C: ChainPayload + Send + Sync + 'static, P: ChainPayload + 'static> chain_drive::in_macro::ChainBlockRef for DelayedDebouncedPersistentCallBlock<C, P> {
    fn insert_into(self_ref: Arc<Mutex<Self>>, chain_drive: &mut ChainDrive) {
        chain_drive.push_front(self_ref.clone() as Arc<Mutex<dyn ChainBlock<C, ChainBFront>>>);
        chain_drive.push_back(self_ref.clone() as Arc<Mutex<dyn ChainBlock<InitPayload, ChainBBack>>>);
    }
}