use chain_drive::{ChainDrive, ChainJumper, ChainJumpResult, define_block, ChainBBack};
use crate::discord::channels::{DiscordDMReceivedPayload, DiscordDMAuthorCrumb, DiscordDMSendPayload};
use crate::discord::DiscordService;
use crate::openai::channels::{LLMCompletionPayload, RunLLMPromptPayload};
use crate::openai::OpenAIService;

mod discord;
mod openai;

#[tokio::main]
async fn main() {
    let mut discord = DiscordService::new().await;
    let openai = OpenAIService::new();

    let mut drive = ChainDrive::new();
    drive.insert(discord.dm_listener_block());
    drive.insert(openai.prompt_runner_block());
    drive.insert(discord.dm_send_block());
    drive.insert(DmLlmAdapter {});
    drive.start();

    discord.start().await;
}

define_block!(
    struct DmLlmAdapter;
    impl for ChainBBack, DiscordDMReceivedPayload {
        fn run(&self, payload: DiscordDMReceivedPayload, jump: ChainJumper<DiscordDMReceivedPayload>) -> ChainJumpResult {
            println!("Received \"{}\"", payload.content);
            jump.to(RunLLMPromptPayload {
                prompt: payload.content
            })
        }
    }
    impl for ChainBBack, LLMCompletionPayload {
        fn run(&self, payload: LLMCompletionPayload, jump: ChainJumper<LLMCompletionPayload>) -> ChainJumpResult {
            let sender = jump.get_crumb::<DiscordDMAuthorCrumb>().unwrap();
            println!("Sending to {}: \"{}\"", sender.author.name, payload.content);
            jump.to(DiscordDMSendPayload {
                content: payload.content,
                user: sender.author.clone(),
                context_http: sender.context_http.clone()
            })
        }
    }
);