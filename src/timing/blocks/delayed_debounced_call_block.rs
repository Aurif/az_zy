use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use chain_drive::{ChainBFront, ChainBlock, ChainDrive, ChainJumper, ChainJumpResult, ChainPayload};
use tokio::task::JoinHandle;
use crate::timing::timing_service::{TimingService};

impl TimingService {
    pub fn delayed_debounced_call_block<C: ChainPayload + Send + Sync + 'static, P: ChainPayload + 'static>(&self, timeout: Duration, func: fn() -> P) -> DelayedDebouncedCallBlock<C, P> {
        DelayedDebouncedCallBlock {
            timeout,
            phantom_data: PhantomData,
            handle: None,
            func,
        }
    }
}

pub struct DelayedDebouncedCallBlock<C: ChainPayload + Send + Sync, P: ChainPayload + 'static> {
    timeout: Duration,
    phantom_data: PhantomData<C>,
    handle: Option<JoinHandle<()>>,
    func: fn() -> P,
}
impl<C: ChainPayload + Send + Sync + 'static, P: ChainPayload + 'static> ChainBlock<C, ChainBFront> for DelayedDebouncedCallBlock<C, P> {
    fn run(&mut self, payload: C, jump: ChainJumper<C>) -> ChainJumpResult {
        if let Some(handle) = &self.handle {
            handle.abort();
        }
        let duration = self.timeout.clone();
        let func = self.func.clone();
        let jump_core = jump.get_core();
        let handle = tokio::spawn(async move {
            tokio::time::sleep(duration).await;
            jump_core.emit(func());
        });
        self.handle = Some(handle);

        jump.next(payload)
    }
}

impl<C: ChainPayload + Send + Sync + 'static, P: ChainPayload + 'static> chain_drive::in_macro::ChainBlockRef for DelayedDebouncedCallBlock<C, P> {
    fn insert_into(self_ref: Arc<Mutex<Self>>, chain_drive: &mut ChainDrive) {
        chain_drive.push_front(self_ref.clone() as Arc<Mutex<dyn ChainBlock<C, ChainBFront>>>);
    }
}