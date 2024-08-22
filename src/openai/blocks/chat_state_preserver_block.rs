use std::fs;
use std::ops::Deref;
use crate::openai::channels::{FullChatHistoryCrumb, LLMCompletionPayload, SystemPromptPayload};
use crate::openai::OpenAIService;
use chain_drive::{
    define_block, ChainBBack, ChainBFront, ChainBlock, ChainJumpResult, ChainJumper,
};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion;
use openai_api_rs::v1::chat_completion::ChatCompletionRequest;
use openai_api_rs::v1::common::{GPT4, GPT4_O, GPT4_O_MINI};
use std::sync::Arc;

impl OpenAIService {
    pub fn chat_state_preserver_block(
        &self,
        summary_prompts: Vec<String>,
    ) -> ChatStatePreserverBlock {
        ChatStatePreserverBlock::new(self.client.clone(), summary_prompts)
    }
}

pub struct ChatStatePreserverBlock {
    history: Arc<FullChatHistoryCrumb>,
    client: Arc<OpenAIClient>,
    summary_prompts: Vec<String>,
}
impl ChatStatePreserverBlock {
    fn new(client: Arc<OpenAIClient>, summary_prompts: Vec<String>) -> ChatStatePreserverBlock {
        ChatStatePreserverBlock {
            history: Arc::new(FullChatHistoryCrumb {
                history: Vec::new(),
            }),
            client,
            summary_prompts,
        }
    }
}

impl ChainBlock<LLMCompletionPayload, ChainBBack> for ChatStatePreserverBlock {
    fn run(
        &mut self,
        payload: LLMCompletionPayload,
        jump: ChainJumper<LLMCompletionPayload>,
    ) -> ChainJumpResult {
        if let Some(history) = jump.get_crumb::<FullChatHistoryCrumb>() {
            self.history = history
        }
        jump.next(payload)
    }
}

impl ChainBlock<SystemPromptPayload, ChainBFront> for ChatStatePreserverBlock {
    fn run(
        &mut self,
        mut payload: SystemPromptPayload,
        jump: ChainJumper<SystemPromptPayload>,
    ) -> ChainJumpResult {
        if let Ok(cache) = fs::read_to_string("./chat_cache.txt") {
            payload.system_prompt.push_str(format!("\n\n{}", cache).deref());
        }
        jump.next(payload)
    }
}

impl Drop for ChatStatePreserverBlock {
    fn drop(&mut self) {
        let mut history = self.history.history.clone();
        let prompts = self.summary_prompts.clone();
        let client = self.client.clone();
        tokio::spawn(async move {
            let mut summary = Vec::new();
            for prompt in prompts {
                history.push(chat_completion::ChatCompletionMessage {
                    role: chat_completion::MessageRole::system,
                    content: chat_completion::Content::Text(prompt),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                });

                let req = ChatCompletionRequest::new(GPT4_O.to_string(), history.clone());
                let result = client.chat_completion(req).await;
                let completion = (&result.unwrap().choices[0].message.content)
                    .clone()
                    .unwrap();

                history.push(chat_completion::ChatCompletionMessage {
                    role: chat_completion::MessageRole::assistant,
                    content: chat_completion::Content::Text(completion.clone()),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                });
                summary.push(completion);
            }
            println!("{}", summary.join("\n\n"));
            fs::write("./chat_cache.txt", summary.join("\n\n")).expect("Unable to write file");
        });
    }
}

define_block!(ChatStatePreserverBlock:
    SystemPromptPayload, ChainBFront;
    LLMCompletionPayload, ChainBBack;
);
