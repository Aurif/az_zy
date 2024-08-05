use std::sync::{Arc, Weak};
use serenity::{async_trait, Client};
use serenity::all::{Context, EventHandler, GatewayIntents, Message};

pub struct DiscordService {
    handler: DiscordServiceHandler
}
impl DiscordService {
    pub async fn new() -> DiscordService {
        DiscordService {
            handler: DiscordServiceHandler {
                message_listeners: Vec::new()
            }
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

    pub fn dm_listener_block(&mut self) -> Arc<DiscordDMListenerBlock> {
        let block = Arc::new(DiscordDMListenerBlock {});
        self.handler.message_listeners.push(Arc::downgrade(&block));
        block
    }
}

struct DiscordServiceHandler {
    message_listeners: Vec<Weak<DiscordDMListenerBlock>>
}
#[async_trait]
impl EventHandler for DiscordServiceHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        for weak_ref in &self.message_listeners {
            if let Some(listener) = weak_ref.upgrade() {
                listener.message(&ctx, &msg)
            }
        }
    }
}

pub struct DiscordDMListenerBlock;
impl DiscordDMListenerBlock {
    fn message(&self, ctx: &Context, msg: &Message) {
        println!("{}", msg.content)
    }
}