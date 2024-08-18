use std::sync::Arc;
use crate::ChainJumper;

pub trait ChainBlock<P: ChainPayload>: Send+Sync {
    fn run(&self, payload: P, next: &dyn Fn(P), jump: &ChainJumper);
}
impl<P: ChainPayload, C: ChainBlock<P>> ChainBlock<P> for Arc<C> {
    fn run(&self, payload: P, next: &dyn Fn(P), jump: &ChainJumper) {
        self.as_ref().run(payload, next, jump)
    }
}

pub trait ChainPayload {}