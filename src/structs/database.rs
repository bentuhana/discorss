use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ServerData {
    pub feed_channel_id: Option<String>,
    pub feeds_list: Option<Vec<FeedsList>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FeedsList {
    pub feed_url: String,
    pub webhook_url: String,
}
