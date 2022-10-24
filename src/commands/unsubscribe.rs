use url::Url;

use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ResolvedOption, ResolvedValue};
use serenity::model::prelude::CommandInteraction;

use crate::database::Database;
use crate::feed::FeedUtils;
use crate::structs::feed::ServerData;

pub async fn run(
    options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();

    let mut db = Database::load(None);
    let guild_id = interaction.guild_id.unwrap().to_string();
    let sub_command = &options.get(0).unwrap().name;

    let data: ServerData;
    if let Some(current_data) = db.get::<ServerData>(&guild_id) {
        if *sub_command == "all" {
            data = ServerData {
                feeds_list: None,
                ..current_data
            };

            db.set(&guild_id, &data).unwrap();
            followup.content("Unsubscribed from all RSS feeds.")
        } else {
            let current_data_clone = current_data.clone();

            let ResolvedValue::SubCommand(from) = &options.get(0).unwrap().value else { return followup.content("Select a subcommand."); };
            let ResolvedValue::String(url) = from.get(0).unwrap().value else { return followup.content("String value not found"); };

            if current_data.feeds_list.is_none() || current_data.feeds_list.unwrap().is_empty() {
                return followup.content("Not subscribed already.");
            }

            if let Ok(url) = Url::parse(url) {
                if !vec!["http", "https"].contains(&url.scheme()) {
                    return followup.content("Entered URL must be using HTTP(S) protocol.");
                }
            } else {
                return followup.content("Entered URL is not valid.");
            }

            if let Ok(feeds) = FeedUtils::get_subscriptions(&guild_id, &db) {
                if !feeds.contains(&url.to_string()) {
                    return followup.content("Not subscribed to <{url}> already.");
                }
            }

            let mut subscriptions = current_data_clone.feeds_list.unwrap();
            let index = subscriptions.iter().position(|feed| feed == url).unwrap();
            subscriptions.remove(index);

            data = ServerData {
                feeds_list: Some(subscriptions),
                ..current_data
            };

            db.set(&guild_id, &data).unwrap();
            followup.content(format!("Unsubscribed from <{url}>"))
        }
    } else {
        followup.content("No subscription already.")
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("unsubscribe")
        .description("Unsubscribe from an RSS feed.")
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "all",
            "Unsubscribe from all RSS feeds.",
        ))
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "from",
                "Unsubscribe from entered RSS feed.",
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::String, "url", "URL of the RSS feed.")
                    .required(true),
            ),
        )
}
