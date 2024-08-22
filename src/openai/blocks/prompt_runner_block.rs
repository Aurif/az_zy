use std::sync::Arc;
use chain_drive::{ChainBBack, ChainJumper, ChainJumpResult, define_block};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::ChatCompletionRequest;
use openai_api_rs::v1::common::GPT4_O_MINI;
use crate::openai::channels::{LLMCompletionPayload, RunLLMPromptPayload};
use crate::openai::OpenAIService;

impl OpenAIService {
    pub fn prompt_runner_block(&self) -> PromptRunnerBlock {
        PromptRunnerBlock { client: self.client.clone() }
    }
}
define_block!(
    pub struct PromptRunnerBlock {
        client: Arc<OpenAIClient>
    }
    impl for ChainBBack, RunLLMPromptPayload {
         fn run(&mut self, payload: RunLLMPromptPayload, jump: ChainJumper<RunLLMPromptPayload>) -> ChainJumpResult {
            let stopped = jump.stop();
            let client = self.client.clone();
            tokio::spawn(async move {
                let req = ChatCompletionRequest::new(
                    GPT4_O_MINI.to_string(),
                    payload.prompt
                );
                let result = client.chat_completion(req).await;
                let completion = (&result.unwrap().choices[0].message.content).clone().unwrap();
                jump.to(LLMCompletionPayload {
                    content: completion
                }).enter();
            });
            stopped
        }
    }
);