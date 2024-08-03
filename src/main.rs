use chain_drive::{ChainDrive, ChainBlock, ChainPayload};
fn main() {
    let start = Payload { history: String::new() };
    let mut drive = ChainDrive::new();
    drive.push_front(BlockA {});
    drive.push_front(BlockB {label: 'B'});
    drive.push_back(BlockB {label: 'C'});
    drive.push_back(BlockB {label: 'D'});
    drive.push_back(BlockB {label: 'E'});
    drive.push_front(BlockA {});
    drive.push_front(BlockC {});
    drive.run(start);
    println!("------");
    let start = Payload2 { history: String::new() };
    drive.run(start);
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

struct BlockC;
impl ChainBlock<Payload2> for BlockC {
    fn run(&self, payload: Payload2, next: &dyn Fn(Payload2)) {
        let new_labels = format!("{}{}", payload.history, "+");
        println!("-> {}", new_labels);
        next(Payload2 { history: new_labels});
    }
}

struct Payload2 {
    history: String
}
impl ChainPayload for Payload2 {}