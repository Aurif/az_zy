use std::sync::{Arc, Mutex};
use crate::core::chain_block::ChainBlock;
use crate::core::chain_drive::ChainJumper;
use crate::core::common::ChainPayload;

pub struct ChainChannel<P: ChainPayload> {
    queue_front: Vec<Arc<Mutex<dyn ChainBlock<P>>>>,
    queue_back: Vec<Arc<Mutex<dyn ChainBlock<P>>>>
}
impl<P: ChainPayload> ChainChannel<P> {
    pub fn new() -> ChainChannel<P> {
        ChainChannel {
            queue_front: Vec::new(),
            queue_back: Vec::new()
        }
    }

    pub fn push_front(&mut self, block: Arc<Mutex<dyn ChainBlock<P>>>) {
        self.queue_front.push(block);
    }
    pub fn push_back(&mut self, block: Arc<Mutex<dyn ChainBlock<P>>>) {
        self.queue_back.push(block);
    }

    pub fn run(&self, initial_payload: P, jumper: &ChainJumper) {
        self.run_at_index(initial_payload, jumper, 0);
    }
    fn run_at_index(&self, payload: P, jumper: &ChainJumper, index: usize) {
        let next = |new_payload: P| {
            self.run_at_index(new_payload, jumper, index + 1)
        };
        match self.queue_front.get(index) {
            Some(block) => {self.run_block(block, payload, &next, jumper)},
            None => {
                if self.queue_back.len() > index - self.queue_front.len() {
                    self.run_block(&self.queue_back[self.queue_back.len() - (index - self.queue_front.len()) - 1], payload, &next, jumper)
                }
            }
        }
    }

    fn run_block(&self, block: &Arc<Mutex<dyn ChainBlock<P>>>, payload: P, next: &dyn Fn(P), jump: &ChainJumper) {
        {
            let guard = block.try_lock().expect("Tried referencing a blocked mutex");
            guard.run(payload, &next, jump)
        }.progress();
    }
}
