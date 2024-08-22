use chain_drive::{ChainDrive, ChainJumper, ChainJumpResult, define_block, ChainBBack, ChainBlock};
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
    drive.insert(openai.constant_system_prompt_block("You are trying to pass the turing test. Keep your messages to bare minimum.".to_string()));
    drive.insert(openai.chat_interface_block());
    drive.insert(openai.prompt_runner_block());
    drive.insert(discord.dm_sender_block());
    drive.start();

    discord.start().await;
}

struct DmLlmAdapter;
impl ChainBlock<DiscordDMReceivedPayload, ChainBBack> for DmLlmAdapter {
    fn run(&mut self, payload: DiscordDMReceivedPayload, jump: ChainJumper<DiscordDMReceivedPayload>) -> ChainJumpResult {
        jump.to(LLMChatMessagePayload { message: payload.content })
    }
}
impl ChainBlock<LLMCompletionPayload, ChainBBack> for DmLlmAdapter {
    fn run(&mut self, payload: LLMCompletionPayload, jump: ChainJumper<LLMCompletionPayload>) -> ChainJumpResult {
        let sender = jump.get_crumb::<DiscordDMAuthorCrumb>().unwrap();
        jump.to(DiscordDMSendPayload {
            content: payload.content,
            user: sender.author.clone(),
            context_http: sender.context_http.clone(),
        })
    }
}
define_block!(DmLlmAdapter:
    DiscordDMReceivedPayload, ChainBBack;
    LLMCompletionPayload, ChainBBack;
);