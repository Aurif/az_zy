use std::sync::{Arc, Mutex};
use crate::{ChainDrive, ChainJumper, ChainPayload};
use crate::core::common::ChainJumpResult;

pub trait ChainBlock<P: ChainPayload>: Send+Sync {
    fn run(&self, payload: P, next: &dyn Fn(P), jump: &ChainJumper) -> ChainJumpResult;
}

impl<P: ChainPayload, C: ChainBlock<P>> ChainBlock<P> for Arc<C> {
    fn run(&self, payload: P, next: &dyn Fn(P), jump: &ChainJumper) -> ChainJumpResult {
        self.as_ref().run(payload, next, jump)
    }
}

pub trait ChainBlockRef {
    fn insert_into(self_ref: Arc<Mutex<Self>>, chain_drive: &mut ChainDrive);
}