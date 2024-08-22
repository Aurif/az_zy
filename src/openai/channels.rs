use chain_drive::{ChainCrumb, ChainPayload};
use openai_api_rs::v1::chat_completion;

pub struct RunLLMPromptPayload {
    pub prompt: Vec<chat_completion::ChatCompletionMessage>
}
impl ChainPayload for RunLLMPromptPayload {}

pub struct LLMChatMessagePayload {
    pub message: String
}
impl ChainPayload for LLMChatMessagePayload {}

pub struct LLMCompletionPayload {
    pub content: String
}
impl ChainPayload for LLMCompletionPayload {}


pub struct SystemPromptPayload {
    pub system_prompt: String
}
impl ChainPayload for SystemPromptPayload {}

pub struct FullChatHistoryCrumb {
    pub prompt: Vec<chat_completion::ChatCompletionMessage>
}
impl ChainCrumb for FullChatHistoryCrumb {}