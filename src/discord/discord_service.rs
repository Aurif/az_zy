use std::sync::{Arc, RwLock, Weak};
use chain_drive::{ChainBlock, ChainJumper, InitPayload};
use serenity::{async_trait, Client};
use serenity::all::{Context, EventHandler, GatewayIntents, Message};
use crate::discord::channels::DiscordDMReceivedPayload;

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
        let block = Arc::new(DiscordDMListenerBlock::new());
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

pub struct DiscordDMListenerBlock {
    chain_jumper: RwLock<Option<ChainJumper>>
}
impl DiscordDMListenerBlock {
    fn new<'a>() -> DiscordDMListenerBlock {
        DiscordDMListenerBlock {
            chain_jumper: RwLock::new(None)
        }
    }
    fn message(&self, _ctx: &Context, msg: &Message) {
        if let Some(jumper) = self.chain_jumper.read().unwrap().as_ref() {
            jumper.to(DiscordDMReceivedPayload {
                content: msg.content.clone()
            })
        }
    }
}
impl ChainBlock<InitPayload> for DiscordDMListenerBlock {
    fn run(&self, payload: InitPayload, next: &dyn Fn(InitPayload), jump: &ChainJumper) {
        println!("Initiated!");
        let mut chain_jumper = self.chain_jumper.write().unwrap();
        *chain_jumper = Some(jump.clone());
        next(payload);
    }
}