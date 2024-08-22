use std::sync::Arc;
use chain_drive::{ChainBFront, ChainJumper, ChainJumpResult, define_block};
use openai_api_rs::v1::chat_completion;
use crate::openai::channels::SystemPromptPayload;
use crate::openai::OpenAIService;

impl OpenAIService {
    pub fn chat_state_preserver_block(&self) -> ChatStatePreserverBlock {
        ChatStatePreserverBlock::new()
    }
}

define_block!(
    pub struct ChatStatePreserverBlock {
        history: Arc<Vec<chat_completion::ChatCompletionMessage>>
    }
    impl {
        fn new() -> ChatStatePreserverBlock {
            ChatStatePreserverBlock {
                history: Arc::new(Vec::new())
            }
        }
    }
    impl for ChainBFront, SystemPromptPayload {
         fn run(&mut self, _payload: SystemPromptPayload, jump: ChainJumper<SystemPromptPayload>) -> ChainJumpResult {
            jump.stop()
        }
    }
);