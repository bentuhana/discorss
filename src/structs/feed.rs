use serde::{Deserialize, Serialize};
use serenity::model::prelude::{ChannelId, WebhookId};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServerData {
    pub feed_channel_id: Option<ChannelId>,
    pub feed_check_interval: Option<i64>,
    pub feed_webhook: Option<FeedWebhook>,
    pub feeds_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeedWebhook {
    pub id: WebhookId,
    pub token: String,
}
