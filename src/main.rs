use chain_drive::{ChainDrive, ChainJumper, ChainJumpResult, define_block, ChainBBack};
use crate::discord::channels::{DiscordDMReceivedPayload, DiscordDMAuthorCrumb, DiscordDMSendPayload};
use crate::discord::DiscordService;
use crate::openai::channels::{LLMChatMessagePayload, LLMCompletionPayload};
use crate::openai::OpenAIService;

mod discord;
mod openai;

#[tokio::main]
async fn main() {
    let mut discord = DiscordService::new().await;
    let openai = OpenAIService::new();

    let mut drive = ChainDrive::new();
    drive.insert(DmLlmAdapter {});
    drive.insert(discord.dm_listener_block());
    drive.insert(openai.chat_interface_block());
    drive.insert(openai.prompt_runner_block());
    drive.insert(discord.dm_sender_block());
    drive.start();

    discord.start().await;
}

define_block!(
    struct DmLlmAdapter;
    impl for ChainBBack, DiscordDMReceivedPayload {
        fn run(&mut self, payload: DiscordDMReceivedPayload, jump: ChainJumper<DiscordDMReceivedPayload>) -> ChainJumpResult {
            jump.to(LLMChatMessagePayload { message: payload.content })
        }
    }
    impl for ChainBBack, LLMCompletionPayload {
        fn run(&mut self, payload: LLMCompletionPayload, jump: ChainJumper<LLMCompletionPayload>) -> ChainJumpResult {
            let sender = jump.get_crumb::<DiscordDMAuthorCrumb>().unwrap();
            jump.to(DiscordDMSendPayload {
                content: payload.content,
                user: sender.author.clone(),
                context_http: sender.context_http.clone()
            })
        }
    }
);