use chain_drive::ChainDrive;
use crate::discord::DiscordService;

mod discord;

#[tokio::main]
async fn main() {
    let mut discord = DiscordService::new().await;

    let mut drive = ChainDrive::new();
    drive.push_front(discord.dm_listener_block());
    drive.start();

    discord.start().await;
}