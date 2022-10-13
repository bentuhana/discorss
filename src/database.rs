use std::env;
use std::fs::{create_dir_all, read_dir};
use std::path::PathBuf;

use pickledb::{PickleDb, PickleDbDumpPolicy};
use serde::{Deserialize, Serialize};
use serenity::model::prelude::ChannelId;

#[derive(Serialize, Deserialize, Default)]
pub struct ServerData {
    pub feed_channel_id: Option<ChannelId>,
    pub feeds_list: Option<Vec<FeedsList>>,
}

#[derive(Serialize, Deserialize)]
pub struct FeedsList {
    pub feed_url: String,
    pub webhook_url: String,
}

pub struct Database;
#[allow(clippy::new_ret_no_self)]
impl Database {
    pub fn new(dump_policy: Option<PickleDbDumpPolicy>) -> PickleDb {
        let database_file_path = env::var("DATABASE_FILE_PATH")
            .expect("Expected DATABASE_FILE_PATH environment variable.");
        let mut database_path = PathBuf::from(&database_file_path);
        database_path.pop();

        if read_dir(&database_path).is_err() {
            match create_dir_all(&database_path) {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            }
        }

        PickleDb::new_json(
            database_file_path,
            dump_policy.unwrap_or(PickleDbDumpPolicy::DumpUponRequest),
        )
    }
}
