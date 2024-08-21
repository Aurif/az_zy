use std::ops::Deref;
use std::sync::Arc;
use chain_drive::{ChainBBack, ChainBFront, ChainJumper, ChainJumpResult, define_block};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O_MINI;
use crate::openai::channels::{LLMChatMessagePayload, LLMCompletionPayload, RunLLMPromptPayload, SystemPromptPayload};

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
        PromptRunnerBlock { client: self.client.clone() }
    }
    pub fn chat_interface_block(&self) -> ChatInterfaceBlock {
        ChatInterfaceBlock::new()
    }
    pub fn constant_system_prompt_block(&self, prompt: String) -> ConstantSystemPromptBlock {
        ConstantSystemPromptBlock { prompt }
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

define_block!(
    pub struct ChatInterfaceBlock {
        history: Vec<String>,
        system_prompt: Option<String>
    }
    impl {
        fn new() -> ChatInterfaceBlock {
            ChatInterfaceBlock {
                history: Vec::new(),
                system_prompt: None
            }
        }
        fn append_history(&mut self, message: String, parity: usize) {
            println!("{} {}", if parity%2 == 0 {"->"} else {"<-"}, message);
            if self.history.len()%2==parity {
                self.history.push(message);
            } else {
                if let Some(last) = self.history.last_mut() {
                    last.push_str(format!("\n{}", message).deref());
                }
            };
        }
        fn form_prompt(&self) -> Vec<chat_completion::ChatCompletionMessage> {
            let mut prompt: Vec<_> = self.history.iter()
                .enumerate()
                .map(|(index, value)| {
                    chat_completion::ChatCompletionMessage {
                        role: if index%2 == 0 {chat_completion::MessageRole::user} else {chat_completion::MessageRole::assistant},
                        content: chat_completion::Content::Text(value.clone()),
                        name: None,
                        tool_calls: None,
                        tool_call_id: None,
                    }
                })
                .collect();
            prompt.insert(0, chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::system,
                content: chat_completion::Content::Text(self.system_prompt.clone().unwrap()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            });
            prompt
        }
    }
    impl for ChainBBack, LLMChatMessagePayload {
         fn run(&mut self, payload: LLMChatMessagePayload, jump: ChainJumper<LLMChatMessagePayload>) -> ChainJumpResult {
            self.append_history(payload.message, 0);
            if self.system_prompt == None {
                return jump.to(SystemPromptPayload{system_prompt: "".to_string()})
            }

            jump.to(RunLLMPromptPayload { prompt: self.form_prompt() } )
        }
    }
    impl for ChainBBack, LLMCompletionPayload {
         fn run(&mut self, payload: LLMCompletionPayload, jump: ChainJumper<LLMCompletionPayload>) -> ChainJumpResult {
            self.append_history(payload.content.clone(), 1);
            jump.next(payload)
         }
    }
    impl for ChainBBack, SystemPromptPayload {
         fn run(&mut self, payload: SystemPromptPayload, jump: ChainJumper<SystemPromptPayload>) -> ChainJumpResult {
            self.system_prompt = Some(payload.system_prompt);
            jump.to(RunLLMPromptPayload { prompt: self.form_prompt() } )
         }
    }
);

define_block!(
    pub struct ConstantSystemPromptBlock {
        prompt: String
    }
    impl for ChainBFront, SystemPromptPayload {
         fn run(&mut self, payload: SystemPromptPayload, jump: ChainJumper<SystemPromptPayload>) -> ChainJumpResult {
            jump.next(SystemPromptPayload {
                system_prompt: self.prompt.clone()
            })
        }
    }
);