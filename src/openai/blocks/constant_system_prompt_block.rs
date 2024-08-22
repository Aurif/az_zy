use chain_drive::{ChainBFront, ChainJumper, ChainJumpResult, define_block};
use crate::openai::channels::SystemPromptPayload;
use crate::openai::OpenAIService;

impl OpenAIService {
    pub fn constant_system_prompt_block(&self, prompt: String) -> ConstantSystemPromptBlock {
        ConstantSystemPromptBlock { prompt }
    }
}

define_block!(
    pub struct ConstantSystemPromptBlock {
        prompt: String
    }
    impl for ChainBFront, SystemPromptPayload {
         fn run(&mut self, _payload: SystemPromptPayload, jump: ChainJumper<SystemPromptPayload>) -> ChainJumpResult {
            jump.next(SystemPromptPayload {
                system_prompt: self.prompt.clone()
            })
        }
    }
);