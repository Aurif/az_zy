use std::any::Any;
use std::sync::{Arc, Mutex};
use chain_drive::{ChainBlock, ChainBlockRef, ChainDrive, ChainJumper, ChainJumpResult, ChainPayload, InitPayload};
use crate::discord::channels::DiscordDMReceivedPayload;
use crate::discord::DiscordService;

mod discord;

#[tokio::main]
async fn main() {
    let mut discord = DiscordService::new().await;

    let mut drive = ChainDrive::new();
    // drive.push_front(discord.dm_listener_block());
    // drive.push_front(DisplayBlock{});
    drive.insert(DisplayBlock{});
    drive.start();

    // discord.start().await;
}

macro_rules! define_block {
    ( struct $name:ident; $(impl for $t:ty { $($code:tt)* })* ) => {
        struct $name;
        $(
            impl ChainBlock<$t> for $name {
                $($code)*
            }
        )*
        impl ChainBlockRef for $name {
            fn insert_into(self_ref: Arc<Mutex<Self>>, chain_drive: &mut ChainDrive) {
                $(
                    chain_drive.push_front(self_ref.clone() as Arc<Mutex<dyn ChainBlock<$t>>>);
                )*
            }
        }
    }
}
define_block!(
    struct DisplayBlock;
    impl for DiscordDMReceivedPayload {
        fn run(&self, payload: DiscordDMReceivedPayload, _next: &dyn Fn(DiscordDMReceivedPayload), jump: &ChainJumper) -> ChainJumpResult {
            println!("Received \"{}\"", payload.content);
            jump.stop()
        }
    }
    impl for InitPayload {
        fn run(&self, _payload: InitPayload, _next: &dyn Fn(InitPayload), jump: &ChainJumper) -> ChainJumpResult {
            println!("Fake init!");
            jump.to(DiscordDMReceivedPayload {content: String::from("More faking!")})
        }
    }
);