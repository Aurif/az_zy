fn main() {
    let mut channel = ChainChannel::new();
    channel.push_front(BlockA {});
    channel.push_front(BlockB {label: 'B'});
    channel.push_back(BlockB {label: 'C'});
    channel.push_back(BlockB {label: 'D'});
    channel.push_back(BlockB {label: 'E'});
    channel.push_front(BlockA {});
    channel.run()
}

struct ChainChannel {
    queue_front: Vec<Box<dyn ChainBlock>>,
    queue_back: Vec<Box<dyn ChainBlock>>
}
impl ChainChannel {
    fn new() -> ChainChannel {
        ChainChannel {
            queue_front: Vec::new(),
            queue_back: Vec::new()
        }
    }

    fn push_front<T: ChainBlock + 'static>(&mut self, block: T) {
        self.queue_front.push(Box::new(block));
    }
    fn push_back<T: ChainBlock + 'static>(&mut self, block: T) {
        self.queue_back.push(Box::new(block));
    }

    fn run(&self) {
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

trait ChainBlock {
    fn run(&self, next: &dyn Fn());
}

struct BlockA {}
impl ChainBlock for BlockA {
    fn run(&self, next: &dyn Fn()) {
        println!("A");
        next();
        next();
    }
}

struct BlockB {
    label: char
}
impl ChainBlock for BlockB {
    fn run(&self, next: &dyn Fn()) {
        println!("{}", self.label);
        next();
    }
}