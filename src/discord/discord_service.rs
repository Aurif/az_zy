use std::sync::{Arc, Mutex, RwLock, Weak};
use chain_drive::{ChainBBack, ChainBFront, ChainJumper, ChainJumperCore, ChainJumpResult, define_block, InitPayload};
use serenity::{async_trait, Client};
use serenity::all::{Context, EventHandler, GatewayIntents, Message};
use crate::discord::channels::{DiscordDMReceivedPayload, DiscordDMAuthorCrumb, DiscordDMSendPayload};

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

    pub fn dm_listener_block(&mut self) -> Arc<Mutex<DiscordDMListenerBlock>> {
        let block = Arc::new(Mutex::new(DiscordDMListenerBlock::new()));
        self.handler.message_listeners.push(Arc::downgrade(&block));
        block
    }

    pub fn dm_sender_block(&mut self) -> DiscordDMSenderBlock {
        DiscordDMSenderBlock {}
    }
}

struct DiscordServiceHandler {
    message_listeners: Vec<Weak<Mutex<DiscordDMListenerBlock>>>
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

define_block!(
    pub struct DiscordDMListenerBlock {
        chain_jumper: RwLock<Option<ChainJumperCore>>
    }
    impl {
        fn new<'a>() -> DiscordDMListenerBlock {
            DiscordDMListenerBlock {
                chain_jumper: RwLock::new(None)
            }
        }
        fn message(&self, ctx: &Context, msg: &Message) {
            if msg.author.bot {return}
            if let Some(jumper) = self.chain_jumper.read().unwrap().as_ref() {
                jumper.add_crumb(
                    DiscordDMAuthorCrumb {
                        author: msg.author.clone(),
                        context_http: ctx.http.clone()
                    }
                ).emit(DiscordDMReceivedPayload {
                    content: msg.content.clone()
                })
            }
        }
    }
    impl for ChainBFront, InitPayload {
        fn run(&mut self, payload: InitPayload, jump: ChainJumper<InitPayload>) -> ChainJumpResult {
            println!("Initiated!");
            let mut chain_jumper = self.chain_jumper.write().unwrap();
            *chain_jumper = Some(jump.get_core());
            jump.next(payload)
        }
    }
);

define_block!(
    pub struct DiscordDMSenderBlock;
    impl for ChainBBack, DiscordDMSendPayload {
        fn run(&mut self, payload: DiscordDMSendPayload, jump: ChainJumper<DiscordDMSendPayload>) -> ChainJumpResult {
            tokio::spawn(async move {
                payload.user.create_dm_channel(&payload.context_http).await
                    .unwrap().say(&payload.context_http, payload.content).await
            });
            jump.stop()
        }
    }
);