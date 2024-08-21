use chain_drive::ChainPayload;

pub struct RunLLMPromptPayload {
    pub prompt: String
}
impl ChainPayload for RunLLMPromptPayload {}

pub struct LLMCompletionPayload {
    pub content: String
}
impl ChainPayload for LLMCompletionPayload {}