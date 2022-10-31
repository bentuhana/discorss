use feed_rs::{
    model::Feed,
    parser::{self, ParseFeedError},
};
use pickledb::PickleDb;

use crate::structs::feed::ServerData;

pub enum GetFeedError {
    AccessError,
    ParseError(ParseFeedError),
}

pub struct FeedUtils;
impl FeedUtils {
    pub async fn get_feed(url: &str) -> Result<Feed, GetFeedError> {
        if let Ok(response) = reqwest::get(url).await {
            let body = response.text().await.unwrap();

            match parser::parse(body.as_bytes()) {
                Ok(parsed) => Ok(parsed),
                Err(parse_err) => Err(GetFeedError::ParseError(parse_err)),
            }
        } else {
            Err(GetFeedError::AccessError)
        }
    }

    pub fn get_subscriptions(guild_id: &str, db: &PickleDb) -> Option<Vec<String>> {
        if let Some(data) = db.get::<ServerData>(guild_id) {
            Some(data.feeds_list.unwrap_or_default())
        } else {
            None
        }
    }
}
