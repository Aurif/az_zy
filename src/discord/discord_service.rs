use std::sync::{Mutex, Weak};
use serenity::{async_trait, Client};
use serenity::all::{Context, EventHandler, GatewayIntents, Message};
use crate::discord::blocks::dm_listener_block::DMListenerBlock;

pub struct DiscordService {
    pub(super) handler: DiscordServiceHandler
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
}

pub(super) struct DiscordServiceHandler {
    pub(super) message_listeners: Vec<Weak<Mutex<DMListenerBlock>>>
}
#[async_trait]
impl EventHandler for DiscordServiceHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        for weak_ref in &self.message_listeners {
            if let Some(listener) = weak_ref.upgrade() {
                listener.try_lock().expect("Cannot call, listener blocked").message(&ctx, &msg)
            }
        }
    }
}
