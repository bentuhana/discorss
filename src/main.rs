use dotenvy::dotenv;
use std::env;

use serenity::model::gateway::GatewayIntents;
use serenity::Client;

mod events;
use events::Events;

mod shard_manager;
use shard_manager::ShardManagerContainer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("BOT_TOKEN").expect("Expected BOT_TOKEN environment variable.");
    let intents = GatewayIntents::empty();

    let mut client = Client::builder(token, intents)
        .event_handler(Events)
        .await
        .expect("Couldn't build client.");

    {
        let mut client_data = client.data.write().await;
        client_data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    if let Err(why) = client.start().await {
        println!("Error occurred when starting the client. {:#?}", why);
    }
}
