use chain_drive::{ChainBBack, ChainJumper, ChainJumpResult, define_block};
use crate::discord::channels::DiscordDMSendPayload;
use crate::discord::DiscordService;

impl DiscordService {
    pub fn dm_sender_block(&mut self) -> DMSenderBlock {
        DMSenderBlock {}
    }
}

define_block!(
    pub struct DMSenderBlock;
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