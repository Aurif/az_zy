pub struct ChainChannel {
    queue_front: Vec<Box<dyn ChainBlock>>,
    queue_back: Vec<Box<dyn ChainBlock>>
}
impl ChainChannel {
    pub fn new() -> ChainChannel {
        ChainChannel {
            queue_front: Vec::new(),
            queue_back: Vec::new()
        }
    }

    pub fn push_front<T: ChainBlock + 'static>(&mut self, block: T) {
        self.queue_front.push(Box::new(block));
    }
    pub fn push_back<T: ChainBlock + 'static>(&mut self, block: T) {
        self.queue_back.push(Box::new(block));
    }

    pub fn run(&self) {
        self.run_at_index(0);
    }
    fn run_at_index(&self, index: usize) {
        let next = || {
            self.run_at_index(index + 1)
        };
        match self.queue_front.get(index) {
            Some(block) => block.run(&next),
            None => {
                if self.queue_back.len() > index - self.queue_front.len() {
                    self.queue_back[self.queue_back.len() - (index - self.queue_front.len()) - 1].run(&next)
                }
            }
        }
    }
}

pub trait ChainBlock {
    fn run(&self, next: &dyn Fn());
}