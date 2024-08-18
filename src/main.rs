use std::any::Any;
use chain_drive::{ChainDrive, ChainJumper, ChainJumpResult, InitPayload, define_block, ChainBBack, ChainBFront};
use crate::discord::channels::DiscordDMReceivedPayload;
use crate::discord::DiscordService;

mod discord;

#[tokio::main]
async fn main() {
    let mut discord = DiscordService::new().await;

    let mut drive = ChainDrive::new();
    drive.insert(DisplayBlock{});
    drive.insert(discord.dm_listener_block());
    drive.start();

    discord.start().await;
}

define_block!(
    struct DisplayBlock;
    impl for ChainBFront, DiscordDMReceivedPayload {
        fn run(&self, payload: DiscordDMReceivedPayload, jump: ChainJumper<DiscordDMReceivedPayload>) -> ChainJumpResult {
            println!("Received \"{}\"", payload.content);
            jump.stop()
        }
    }
    impl for ChainBBack, InitPayload {
        fn run(&self, _payload: InitPayload, jump: ChainJumper<InitPayload>) -> ChainJumpResult {
            println!("Fake init!");
            jump.to(DiscordDMReceivedPayload {content: String::from("More faking!")})
        }
    }
);