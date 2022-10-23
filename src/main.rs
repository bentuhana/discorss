use std::env;
use std::fs::File;

use dotenvy::dotenv;

use serenity::model::gateway::GatewayIntents;
use serenity::Client;

mod events;
use events::Events;

mod structs;
use structs::shard_manager::ShardManagerContainer;

mod database;
use database::Database;

mod commands;
mod feed;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("BOT_TOKEN").expect("Expected BOT_TOKEN environment variable.");
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_WEBHOOKS;

    let mut client = Client::builder(token, intents)
        .event_handler(Events)
        .await
        .expect("Could not build client.");

    {
        let mut client_data = client.data.write().await;
        client_data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let database_file_path =
        env::var("DATABASE_FILE_PATH").expect("Expected DATABASE_FILE_PATH environment variable.");
    if File::open(&database_file_path).is_err() {
        if let Err(why) = Database::new(&database_file_path) {
            println!("Error occurred when creating database file. {:#?}", why);
        }
    }

    if let Err(why) = client.start().await {
        println!("Error occurred when starting the client. {:#?}", why);
    }
}
