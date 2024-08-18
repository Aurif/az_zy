use std::any::Any;
use chain_drive::{ChainDrive, ChainJumper, ChainJumpResult, InitPayload, define_block};
use crate::discord::channels::DiscordDMReceivedPayload;
use crate::discord::DiscordService;

mod discord;

#[tokio::main]
async fn main() {
    let mut discord = DiscordService::new().await;

    let mut drive = ChainDrive::new();
    drive.insert(discord.dm_listener_block());
    drive.insert(DisplayBlock{});
    drive.start();

    discord.start().await;
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