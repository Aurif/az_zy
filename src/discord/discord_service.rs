use serenity::{async_trait, Client};
use serenity::all::{Context, EventHandler, GatewayIntents, Message};

pub struct DiscordService {
    handler: DiscordServiceHandler
}
impl DiscordService {
    pub async fn new() -> DiscordService {
        DiscordService {
            handler: DiscordServiceHandler {}
        }
    }

    pub async fn start(self) {
        let token = env!("DISCORD_TOKEN");
        let intents = GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        let mut client = Client::builder(&token, intents)
            .event_handler(self.handler)
            .await.expect("Err creating discord client");

        if let Err(why) = client.start().await {
            println!("Discord client error: {why:?}");
        }
    }
}

struct DiscordServiceHandler;
#[async_trait]
impl EventHandler for DiscordServiceHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "uwu!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}
