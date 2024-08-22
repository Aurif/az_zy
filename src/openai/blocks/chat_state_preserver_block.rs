use std::sync::Arc;
use chain_drive::{ChainBFront, ChainBlock, ChainJumper, ChainJumpResult, define_block};
use openai_api_rs::v1::chat_completion;
use crate::openai::channels::SystemPromptPayload;
use crate::openai::OpenAIService;

impl OpenAIService {
    pub fn chat_state_preserver_block(&self) -> ChatStatePreserverBlock {
        ChatStatePreserverBlock::new()
    }
}

pub struct ChatStatePreserverBlock {
    history: Arc<Vec<chat_completion::ChatCompletionMessage>>
}
impl ChatStatePreserverBlock {
    fn new() -> ChatStatePreserverBlock {
        ChatStatePreserverBlock {
            history: Arc::new(Vec::new())
        }
    }
}

impl ChainBlock<SystemPromptPayload, ChainBFront> for ChatStatePreserverBlock{
     fn run(&mut self, _payload: SystemPromptPayload, jump: ChainJumper<SystemPromptPayload>) -> ChainJumpResult {
        jump.stop()
    }
}

define_block!(ChatStatePreserverBlock: SystemPromptPayload, ChainBFront);