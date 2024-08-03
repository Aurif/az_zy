use chain_drive::{ChainChannel, ChainBlock, ChainPayload};
fn main() {
    let start = Payload { history: String::new() };
    let mut channel = ChainChannel::new();
    channel.push_front(BlockA {});
    channel.push_front(BlockB {label: 'B'});
    channel.push_back(BlockB {label: 'C'});
    channel.push_back(BlockB {label: 'D'});
    channel.push_back(BlockB {label: 'E'});
    channel.push_front(BlockA {});
    channel.run(start)
}



struct BlockA {}
impl ChainBlock<Payload> for BlockA {
    fn run(&self, payload: Payload, next: &dyn Fn(Payload)) {
        let new_labels = format!("{}{}", payload.history, "A");
        println!("-> {}", new_labels);
        next(Payload { history: new_labels});
        next(Payload { history: format!("{}{}", payload.history, "A2")});
    }
}

struct BlockB {
    label: char
}
impl ChainBlock<Payload> for BlockB {
    fn run(&self, payload: Payload, next: &dyn Fn(Payload)) {
        let new_labels = format!("{}{}", payload.history, self.label);
        println!("-> {}", new_labels);
        next(Payload { history: new_labels});
    }
}

struct Payload {
    history: String
}
impl ChainPayload for Payload {}