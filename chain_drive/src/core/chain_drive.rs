use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, RwLock, Weak};
use crate::{ChainCrumb, ChainJumpResult};
use crate::core::chain_block::{ChainBBack, ChainBFront, ChainBlock, ChainBlockInserter};
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

    pub fn push_front<P: ChainPayload + 'static>(&mut self, block: Arc<Mutex<dyn ChainBlock<P, ChainBFront>>>) {
        self.core.write().unwrap().get_channel_mut().push_front(block)
    }

    pub fn push_back<P: ChainPayload + 'static>(&mut self, block: Arc<Mutex<dyn ChainBlock<P, ChainBBack>>>) {
        self.core.write().unwrap().get_channel_mut().push_back(block)
    }

    pub fn insert(&mut self, block: impl ChainBlockInserter) {
        block.insert_into(self)
    }

    pub fn start(&self) {
        let jumper = ChainJumperCore::new(
            Arc::downgrade(&self.core)
        );
        jumper.direct_to(InitPayload {}, 0).enter()
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
pub struct ChainJumperCore {
    drive_core: Weak<RwLock<ChainDriveCore>>,
    crumbs: HashMap<TypeId, Arc<dyn Any+Send+Sync>>
}
impl ChainJumperCore {
    fn new(core: Weak<RwLock<ChainDriveCore>>) -> ChainJumperCore {
        ChainJumperCore {
            drive_core: core,
            crumbs: HashMap::new()
        }
    }

    fn direct_to<P: ChainPayload + 'static>(&self, payload: P, index: usize) -> ChainJumpResult {
        if let Some(core) = self.drive_core.upgrade() {
            let self_clone = self.clone();
            return ChainJumpResult::from_func(Box::new(move || {
                core.read().unwrap().get_channel().run_at_index(payload, &self_clone, index)
            }))
        }
        ChainJumpResult::from_blank()
    }

    pub fn emit<P: ChainPayload + 'static>(&self, payload: P) {
        self.direct_to(payload, 0).enter()
    }

    pub(crate) fn arm<P: ChainPayload>(&self, next_index: usize) -> ChainJumper<P> {
        ChainJumper::<P> {
            core: self.clone(),
            next_index,
            phantom: PhantomData
        }
    }

    pub fn add_crumb<C: ChainCrumb>(&self, crumb: C) -> ChainJumperCore {
        let mut jumper = self.clone();
        jumper.crumbs.insert(TypeId::of::<C>(), Arc::new(crumb));
        jumper
    }
}

pub struct ChainJumper<N: ChainPayload> {
    core: ChainJumperCore,
    next_index: usize,
    phantom: PhantomData<N>,
}
impl<N: ChainPayload + 'static> ChainJumper<N> {
    pub fn to<P: ChainPayload + 'static>(&self, payload: P) -> ChainJumpResult {
        self.core.direct_to(payload, 0)
    }

    pub fn stop(&self) -> ChainJumpResult {
        ChainJumpResult::from_blank()
    }

    pub fn next(&self, payload: N) -> ChainJumpResult {
        self.core.direct_to(payload, self.next_index)
    }

    pub fn get_core(&self) -> ChainJumperCore {
        self.core.clone()
    }

    pub fn get_crumb<C: ChainCrumb>(&self) -> Option<Arc<C>> {
        self.core.crumbs.get(&TypeId::of::<C>()).and_then(|c| {
            let arc_any = c.clone();
            let downcasted = arc_any.downcast::<C>().ok()?;
            Some(downcasted)
        })
    }

    pub fn add_crumb<C: ChainCrumb>(&self, crumb: C) -> ChainJumper<N> {
        ChainJumper {
            core: self.core.add_crumb(crumb),
            next_index: self.next_index.clone(),
            phantom: PhantomData,
        }
    }
}

pub struct InitPayload;
impl ChainPayload for InitPayload {}