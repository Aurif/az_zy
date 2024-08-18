use chain_drive::{ChainBlock, ChainDrive, ChainJumper};
use crate::discord::channels::DiscordDMReceivedPayload;
use crate::discord::DiscordService;

mod discord;

#[tokio::main]
async fn main() {
    let mut discord = DiscordService::new().await;

    let mut drive = ChainDrive::new();
    drive.push_front(discord.dm_listener_block());
    drive.push_front(DisplayBlock{});
    drive.start();

    discord.start().await;
}

struct DisplayBlock;
impl ChainBlock<DiscordDMReceivedPayload> for DisplayBlock {
    fn run(&self, payload: DiscordDMReceivedPayload, _next: &dyn Fn(DiscordDMReceivedPayload), _jump: &ChainJumper) {
        println!("Received \"{}\"", payload.content);
    }
}