use chain_drive::ChainPayload;

pub struct DiscordDMReceivedPayload {
    pub content: String
}
impl ChainPayload for DiscordDMReceivedPayload {}