use url::Url;

use serenity::builder::{
    AutocompleteChoice, CreateAutocompleteResponse, CreateCommand, CreateCommandOption,
    CreateInteractionResponseFollowup,
};
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

    let followup_content = match db.get::<ServerData>(&guild_id) {
        Some(current_data) => {
            if *sub_command == "all" {
                db.set(
                    &guild_id,
                    &ServerData {
                        feeds_list: None,
                        ..current_data
                    },
                )
                .unwrap();

                "Unsubscribed from all RSS feeds.".to_owned()
            } else {
                if current_data.feeds_list.is_none()
                    || current_data.feeds_list.as_ref().unwrap().is_empty()
                {
                    return followup.content("Not subscribed already.");
                }

                let ResolvedValue::SubCommand(from) = &options.get(0).unwrap().value else { return followup.content("Select a subcommand."); };
                let ResolvedValue::String(url) = from.get(0).unwrap().value else { return followup.content("String value not found"); };

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

                let mut subscriptions = current_data.feeds_list.unwrap();
                let index = subscriptions.iter().position(|feed| feed == url).unwrap();
                subscriptions.remove(index);

                db.set(
                    &guild_id,
                    &ServerData {
                        feeds_list: Some(subscriptions),
                        ..current_data
                    },
                )
                .unwrap();

                format!("Unsubscribed from <{url}>")
            }
        }
        None => "No subscription already.".to_owned(),
    };

    followup.content(followup_content)
}

pub fn autocomplete(interaction: &CommandInteraction) -> CreateAutocompleteResponse {
    let autocomplete = CreateAutocompleteResponse::new();
    let db = Database::load(None);
    let guild_id = interaction.guild_id.unwrap().to_string();

    let feeds_list = match db.get::<ServerData>(&guild_id) {
        Some(current_data) => {
            // TODO: use Option::is_some_and() when it lands to Rust.
            // * Tracking issue: https://github.com/rust-lang/rust/issues/93050
            if matches!(current_data.feeds_list.as_ref(), Some(list) if !list.is_empty()) {
                current_data.feeds_list.unwrap()
            } else {
                vec![]
            }
        }
        None => vec![],
    };

    let mut choices: Vec<AutocompleteChoice> = vec![];
    if !feeds_list.is_empty() {
        for feed in feeds_list {
            choices.push(AutocompleteChoice {
                name: feed.to_owned(),
                value: serenity::json::Value::String(feed),
            })
        }
    }

    autocomplete.set_choices(choices)
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
                    .required(true)
                    .set_autocomplete(true),
            ),
        )
}
