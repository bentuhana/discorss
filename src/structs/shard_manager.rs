use std::sync::Arc;

use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::{Mutex, TypeMapKey};

pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
