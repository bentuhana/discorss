use serde::{Deserialize, Serialize};
use serenity::model::prelude::{ChannelId, WebhookId};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ServerData {
    pub feed_channel_id: Option<ChannelId>,
    pub feed_webhook: Option<FeedWebhook>,
    pub feeds_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeedWebhook {
    pub id: WebhookId,
    pub token: String,
}
