use std::ops::Deref;
use chain_drive::{ChainBBack, ChainBlock, ChainJumper, ChainJumpResult, define_block};
use openai_api_rs::v1::chat_completion;
use crate::openai::channels::{FullChatHistoryCrumb, LLMChatMessagePayload, LLMCompletionPayload, RunLLMPromptPayload, SystemPromptPayload};
use crate::openai::OpenAIService;

impl OpenAIService {
    pub fn chat_interface_block(&self) -> ChatInterfaceBlock {
        ChatInterfaceBlock::new()
    }
}

pub struct ChatInterfaceBlock {
    history: Vec<String>,
    system_prompt: Option<String>,
}
impl ChatInterfaceBlock {
    fn new() -> ChatInterfaceBlock {
        ChatInterfaceBlock {
            history: Vec::new(),
            system_prompt: None,
        }
    }
    fn append_history(&mut self, message: String, parity: usize) {
        println!("{} {}", if parity % 2 == 0 { "->" } else { "<-" }, message);
        if self.history.len() % 2 == parity {
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
                    role: if index % 2 == 0 { chat_completion::MessageRole::user } else { chat_completion::MessageRole::assistant },
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

impl ChainBlock<LLMChatMessagePayload, ChainBBack> for ChatInterfaceBlock {
    fn run(&mut self, payload: LLMChatMessagePayload, jump: ChainJumper<LLMChatMessagePayload>) -> ChainJumpResult {
        self.append_history(payload.message, 0);
        if self.system_prompt == None {
            return jump.to(SystemPromptPayload { system_prompt: "".to_string() });
        }

        jump.to(RunLLMPromptPayload { prompt: self.form_prompt() })
    }
}

impl ChainBlock<LLMCompletionPayload, ChainBBack> for ChatInterfaceBlock {
    fn run(&mut self, payload: LLMCompletionPayload, jump: ChainJumper<LLMCompletionPayload>) -> ChainJumpResult {
        self.append_history(payload.content.clone(), 1);
        jump.add_crumb(FullChatHistoryCrumb { history: self.form_prompt() }).next(payload)
    }
}

impl ChainBlock<SystemPromptPayload, ChainBBack> for ChatInterfaceBlock {
    fn run(&mut self, payload: SystemPromptPayload, jump: ChainJumper<SystemPromptPayload>) -> ChainJumpResult {
        self.system_prompt = Some(payload.system_prompt);
        jump.to(RunLLMPromptPayload { prompt: self.form_prompt() })
    }
}

define_block!(ChatInterfaceBlock:
    LLMChatMessagePayload, ChainBBack;
    LLMCompletionPayload, ChainBBack;
    SystemPromptPayload, ChainBBack;
);