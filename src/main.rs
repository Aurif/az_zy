use chain_drive::{ChainChannel, ChainBlock};
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