use feed_rs::parser::ParseFeedError;
use url::Url;

use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ResolvedOption, ResolvedValue};
use serenity::model::prelude::CommandInteraction;

use crate::database::Database;
use crate::feed::{FeedUtils, GetFeedError};
use crate::structs::feed::ServerData;

pub async fn run(
    options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();

    let mut db = Database::load(None);
    let guild_id = interaction.guild_id.unwrap().to_string();
    let ResolvedValue::String(url) = &options.get(0).unwrap().value else { return followup.content("String value not found"); };

    if let Ok(url) = Url::parse(url) {
        if !vec!["http", "https"].contains(&url.scheme()) {
            return followup.content("Entered URL must be using HTTP(S) protocol.");
        }
    } else {
        return followup.content("Entered URL is not valid.");
    }

    if let Ok(feeds) = FeedUtils::get_subscriptions(&guild_id, &db) {
        if feeds.contains(&url.to_string()) {
            return followup.content(format!("Already subscribed to <{url}>."));
        }
    }

    let data = match FeedUtils::get_feed(url).await {
        Ok(_) => {
            if let Some(current_data) = db.get::<ServerData>(&guild_id) {
                let mut feeds_list = current_data.feeds_list.unwrap_or_default();
                feeds_list.push(url.to_string());

                ServerData {
                    feeds_list: Some(feeds_list),
                    ..current_data
                }
            } else {
                ServerData {
                    feeds_list: Some(vec![url.to_string()]),
                    ..Default::default()
                }
            }
        }
        Err(err) => {
            let reason = match err {
                GetFeedError::AccessError => "Cannot access to given URL.",
                GetFeedError::ParseError(parse_err) => match parse_err {
                    ParseFeedError::IoError(_) => "An error occured while reading RSS file. If this keeps happening, please report the issue to the developer.",
                    ParseFeedError::JsonSerde(_) => "An error occurred while reading JSON content. If this keeps happening, please report the issue to the developer.",
                    ParseFeedError::JsonUnsupportedVersion(_) => "Unsupported JSON version on feed content.",
                    ParseFeedError::ParseError(_) => "Entered URL is not an RSS feed.",
                    ParseFeedError::XmlReader(_) => "RSS content is broken on entered feed."
                },
            };

            return followup.content(format!("Cannot subscribe to <{url}>. {reason}"));
        }
    };

    db.set(&guild_id, &data).unwrap();
    followup.content(format!("Subscribed to <{url}>."))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("subscribe")
        .description("Subscribe to an RSS feed.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "url", "URL of the RSS feed.")
                .required(true),
        )
}
