use std::sync::Arc;
use chain_drive::{ChainBBack, ChainJumper, ChainJumpResult, define_block};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O_MINI;
use crate::openai::channels::{LLMCompletionPayload, RunLLMPromptPayload};

pub struct OpenAIService {
    client: Arc<OpenAIClient>
}
impl OpenAIService {
    pub fn new() -> OpenAIService {
        OpenAIService {
            client: Arc::new(OpenAIClient::new(env!("OPENAI_TOKEN").to_string()))
        }
    }

    pub fn prompt_runner_block(&self) -> PromptRunnerBlock {
        PromptRunnerBlock {
            client: self.client.clone()
        }
    }
}
define_block!(
    pub struct PromptRunnerBlock {
        client: Arc<OpenAIClient>
    }
    impl for ChainBBack, RunLLMPromptPayload {
         fn run(&self, payload: RunLLMPromptPayload, jump: ChainJumper<RunLLMPromptPayload>) -> ChainJumpResult {
            let stopped = jump.stop();
            let client = self.client.clone();
            tokio::spawn(async move {
                let req = ChatCompletionRequest::new(
                    GPT4_O_MINI.to_string(),
                    vec![chat_completion::ChatCompletionMessage {
                        role: chat_completion::MessageRole::system,
                        content: chat_completion::Content::Text(payload.prompt),
                        name: None,
                        tool_calls: None,
                        tool_call_id: None,
                    }],
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