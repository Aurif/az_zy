pub struct ChainChannel<P: ChainPayload> {
    queue_front: Vec<Box<dyn ChainBlock<P>>>,
    queue_back: Vec<Box<dyn ChainBlock<P>>>
}
impl<P: ChainPayload> ChainChannel<P> {
    pub fn new() -> ChainChannel<P> {
        ChainChannel {
            queue_front: Vec::new(),
            queue_back: Vec::new()
        }
    }

    pub fn push_front<T: ChainBlock<P> + 'static>(&mut self, block: T) {
        self.queue_front.push(Box::new(block));
    }
    pub fn push_back<T: ChainBlock<P> + 'static>(&mut self, block: T) {
        self.queue_back.push(Box::new(block));
    }

    pub fn run(&self, initial_payload: P) {
        self.run_at_index(initial_payload, 0);
    }
    fn run_at_index(&self, payload: P, index: usize) {
        let next = |new_payload: P| {
            self.run_at_index(new_payload, index + 1)
        };
        match self.queue_front.get(index) {
            Some(block) => block.run(payload, &next),
            None => {
                if self.queue_back.len() > index - self.queue_front.len() {
                    self.queue_back[self.queue_back.len() - (index - self.queue_front.len()) - 1].run(payload, &next)
                }
            }
        }
    }
}

pub trait ChainBlock<P: ChainPayload> {
    fn run(&self, payload: P, next: &dyn Fn(P));
}

pub trait ChainPayload {}