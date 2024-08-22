use std::sync::Arc;
use openai_api_rs::v1::api::OpenAIClient;

pub struct OpenAIService {
    pub(super) client: Arc<OpenAIClient>
}
impl OpenAIService {
    pub fn new() -> OpenAIService {
        OpenAIService {
            client: Arc::new(OpenAIClient::new(env!("OPENAI_TOKEN").to_string()))
        }
    }
}