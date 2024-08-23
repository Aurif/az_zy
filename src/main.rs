use std::time::Duration;
use chain_drive::{ChainDrive, ChainJumper, ChainJumpResult, define_block, ChainBBack, ChainBlock};
use crate::discord::channels::{DiscordDMReceivedPayload, DiscordDMAuthorCrumb, DiscordDMSendPayload};
use crate::discord::DiscordService;
use crate::openai::channels::{LLMChatMessagePayload, LLMCompletionPayload, ClearStatePayload};
use crate::openai::OpenAIService;
use crate::timing::TimingService;

mod discord;
mod openai;
mod timing;

#[tokio::main]
async fn main() {
    let mut discord = DiscordService::new().await;
    let openai = OpenAIService::new();
    let timing = TimingService::new();

    let mut drive = ChainDrive::new();
    drive.insert(DmLlmAdapter {});
    drive.insert(discord.dm_listener_block());
    drive.insert(openai.constant_system_prompt_block("You are trying to pass the turing test. Keep your messages to bare minimum. Use brief, friendly tone.".to_string()));
    drive.insert(openai.chat_state_preserver_block(vec!("Based on the conversation, create a prompt that will help future LLM in conversation with this user. Keep only the most useful information. Reply only with the contents of the prompt.".to_string(), "Extract newly learnt information from the conversation. Reply only with those information.".to_string())));
    drive.insert(openai.chat_interface_block());
    drive.insert(openai.prompt_runner_block());
    drive.insert(discord.dm_sender_block());
    drive.insert(timing.delayed_debounced_call_block::<DiscordDMReceivedPayload, _>(Duration::from_secs(30), || { ClearStatePayload {} }));
    drive.start();

    tokio::join!(
        discord.start()
    );
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