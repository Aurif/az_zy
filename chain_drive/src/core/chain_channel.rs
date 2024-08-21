use std::sync::{Arc, Mutex};
use crate::core::chain_block::{ChainB, ChainBBack, ChainBFront, ChainBlock};
use crate::core::chain_drive::{ChainJumper, ChainJumperCore};
use crate::core::common::ChainPayload;

pub struct ChainChannel<P: ChainPayload> {
    queue_front: Vec<Arc<Mutex<dyn ChainBlock<P, ChainBFront>>>>,
    queue_back: Vec<Arc<Mutex<dyn ChainBlock<P, ChainBBack>>>>
}
impl<P: ChainPayload> ChainChannel<P> {
    pub fn new() -> ChainChannel<P> {
        ChainChannel {
            queue_front: Vec::new(),
            queue_back: Vec::new()
        }
    }

    pub fn push_front(&mut self, block: Arc<Mutex<dyn ChainBlock<P, ChainBFront>>>) {
        self.queue_front.push(block);
    }
    pub fn push_back(&mut self, block: Arc<Mutex<dyn ChainBlock<P, ChainBBack>>>) {
        self.queue_back.push(block);
    }

    pub fn run(&self, initial_payload: P, jumper: &ChainJumperCore) {
        self.run_at_index(initial_payload, jumper, 0);
    }
    pub(crate) fn run_at_index(&self, payload: P, jumper: &ChainJumperCore, index: usize) {
        let jumper = jumper.arm(index + 1);
        match self.queue_front.get(index) {
            Some(block) => {self.run_block(block, payload, jumper)},
            None => {
                if self.queue_back.len() > index - self.queue_front.len() {
                    self.run_block(&self.queue_back[self.queue_back.len() - (index - self.queue_front.len()) - 1], payload, jumper)
                }
            }
        }
    }

    fn run_block<B: ChainB>(&self, block: &Arc<Mutex<dyn ChainBlock<P, B>>>, payload: P, jump: ChainJumper<P>) {
        {
            let mut guard = block.try_lock().expect("Tried referencing a blocked mutex");
            guard.run(payload, jump)
        }.enter();
    }
}
