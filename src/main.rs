use crate::discord::DiscordService;

mod discord;

#[tokio::main]
async fn main() {
    let mut discord = DiscordService::new().await;

    let block = discord.dm_listener_block();

    discord.start().await;
}