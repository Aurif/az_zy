use std::sync::{Arc, Mutex, RwLock};
use chain_drive::{ChainBFront, ChainJumper, ChainJumperCore, ChainJumpResult, define_block, InitPayload};
use serenity::all::{Context, Message};
use crate::discord::channels::{DiscordDMAuthorCrumb, DiscordDMReceivedPayload};
use crate::discord::DiscordService;

impl DiscordService {
    pub fn dm_listener_block(&mut self) -> Arc<Mutex<DMListenerBlock>> {
        let block = Arc::new(Mutex::new(DMListenerBlock::new()));
        self.handler.message_listeners.push(Arc::downgrade(&block));
        block
    }
}

define_block!(
    pub struct DMListenerBlock {
        chain_jumper: RwLock<Option<ChainJumperCore>>
    }
    impl {
        fn new<'a>() -> DMListenerBlock {
            DMListenerBlock {
                chain_jumper: RwLock::new(None)
            }
        }
        pub(in crate::discord) fn message(&self, ctx: &Context, msg: &Message) {
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