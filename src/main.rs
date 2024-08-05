use crate::discord::DiscordService;

mod discord;

#[tokio::main]
async fn main() {
    let discord = DiscordService::new().await;
    discord.start().await;
}