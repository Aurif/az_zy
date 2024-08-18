use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, Weak};
use crate::{ChainJumpResult};
use crate::core::chain_block::{ChainBlock, ChainBlockInserter};
use crate::core::ChainChannel;
use crate::core::common::ChainPayload;

pub struct ChainDrive {
    core: Arc<RwLock<ChainDriveCore>>
}
impl ChainDrive {
    pub fn new() -> ChainDrive {
        let core = Arc::new(RwLock::new(ChainDriveCore::new()));
        return ChainDrive { core }
    }

    pub fn push_front<P: ChainPayload + 'static>(&mut self, block: Arc<Mutex<dyn ChainBlock<P>>>) {
        self.core.write().unwrap().get_channel_mut().push_front(block)
    }

    pub fn push_back<P: ChainPayload + 'static>(&mut self, block: Arc<Mutex<dyn ChainBlock<P>>>) {
        self.core.write().unwrap().get_channel_mut().push_back(block)
    }

    pub fn insert(&mut self, block: impl ChainBlockInserter) {
        block.insert_into(self)
    }

    pub fn start(&self) {
        let jumper = ChainJumper {
            core: Arc::downgrade(&self.core)
        };
        jumper.to(InitPayload {}).progress()
    }
}

pub struct ChainDriveCore {
    channels: HashMap<TypeId, Box<dyn Any + Send + Sync>>
}
impl ChainDriveCore {
    fn new() -> ChainDriveCore {
        return ChainDriveCore { channels: HashMap::new() }
    }

    fn get_channel_mut<P: ChainPayload + 'static>(&mut self) -> &mut ChainChannel<P> {
        let entry = self.channels
            .entry(TypeId::of::<P>())
            .or_insert_with(|| Box::new(ChainChannel::<P>::new()));

        if let Some(channel) = entry.downcast_mut::<ChainChannel<P>>() {
            return channel
        }
        panic!("Channel type mismatch for {}", std::any::type_name::<P>());
    }

    fn get_channel<P: ChainPayload + 'static>(&self) -> &ChainChannel<P> {
        let entry = self.channels
            .get(&TypeId::of::<P>())
            .expect(&format!("Channel missing for {}", std::any::type_name::<P>()));

        if let Some(channel) = entry.downcast_ref::<ChainChannel<P>>() {
            return channel;
        }
        panic!("Channel type mismatch for {}", std::any::type_name::<P>());
    }

}

#[derive(Clone)]
pub struct ChainJumper {
    core: Weak<RwLock<ChainDriveCore>>
}
impl ChainJumper {
    pub fn to<P: ChainPayload + 'static>(&self, payload: P) -> ChainJumpResult {
        if let Some(core) = self.core.upgrade() {
            let self_clone = self.clone();
            return ChainJumpResult::from_func(Box::new(move || {core.read().unwrap().get_channel().run(payload, &self_clone)}))
        }
        ChainJumpResult::from_blank()
    }

    pub fn stop(&self) -> ChainJumpResult {
        ChainJumpResult::from_blank()
    }
}

pub struct InitPayload;
impl ChainPayload for InitPayload {}