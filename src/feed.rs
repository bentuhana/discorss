use std::num::NonZeroU64;

use feed_rs::{
    model::Feed,
    parser::{self, ParseFeedError},
};
use pickledb::PickleDb;
use reqwest::IntoUrl;

use crate::structs::database::ServerData;

pub enum GetFeedError {
    AccessError,
    ParseError(ParseFeedError),
}

pub struct FeedUtils;
impl FeedUtils {
    pub async fn get_feed<U: IntoUrl>(url: U) -> Result<Feed, GetFeedError> {
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

    pub fn get_subscriptions(guild_id: NonZeroU64, db: &PickleDb) -> Result<Vec<String>, ()> {
        if let Some(data) = db.get::<ServerData>(guild_id.to_string().as_str()) {
            let mut feeds = vec![];

            if let Some(feeds_list) = data.feeds_list {
                for feed in feeds_list.into_iter() {
                    feeds.push(feed.feed_url)
                }
            }

            Ok(feeds)
        } else {
            Err(())
        }
    }
}
