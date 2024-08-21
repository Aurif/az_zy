use std::sync::Arc;
use chain_drive::{ChainCrumb, ChainPayload};
use serenity::all::{Http, User};

pub struct DiscordDMReceivedPayload {
    pub content: String
}
impl ChainPayload for DiscordDMReceivedPayload {}


pub struct DiscordDMSendPayload {
    pub content: String,
    pub user: User,
    pub context_http: Arc<Http>
}
impl ChainPayload for DiscordDMSendPayload {}

pub struct DiscordDMAuthorCrumb {
    pub author: User,
    pub context_http: Arc<Http>
}
impl ChainCrumb for DiscordDMAuthorCrumb {}