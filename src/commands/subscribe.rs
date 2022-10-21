use feed_rs::parser::ParseFeedError;
use url::Url;

use serenity::builder::{
    CreateApplicationCommand, CreateApplicationCommandOption, CreateInteractionResponseFollowup,
    CreateWebhook,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ResolvedOption, ResolvedValue};
use serenity::model::prelude::{ApplicationCommandInteraction, ChannelId};
use serenity::prelude::Context;

use crate::database::{Database, FeedsList, ServerData};
use crate::feed::{FeedUtils, GetFeedError};

pub async fn run(
    options: &[ResolvedOption<'_>],
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();
    let guild_id = interaction.guild_id.unwrap().to_string();

    let mut db = Database::load(None);

    let ResolvedValue::String(url) = &options.get(0).unwrap().value else { return followup.content("String value not found"); };

    if let Ok(url) = Url::parse(url) {
        if url.scheme() != "http" && url.scheme() != "https" {
            return followup.content("The URL entered must be using the HTTP(S) protocol.");
        }
    } else {
        return followup.content("Entered URL is not valid.");
    }

    if let Ok(feeds) = FeedUtils::get_subscriptions(guild_id.parse().unwrap(), &db) {
        if feeds.contains(&url.to_string()) {
            return followup.content(format!("Already subscribed to {url}."));
        }
    }

    match FeedUtils::get_feed(url.to_string()).await {
        Ok(feed) => {
            let feed_title = feed.title.unwrap().content;

            let prev_data = db.get::<ServerData>(guild_id.as_str());

            if let Some(data) = prev_data {
                let cloned_data = data.clone();

                let mut feeds_list = cloned_data.feeds_list.unwrap_or_default();
                let feed_channel_id = cloned_data.feed_channel_id.unwrap().parse().unwrap();

                let webhook = CreateWebhook::new(feed_title)
                    .execute(&ctx.http, ChannelId(feed_channel_id))
                    .await;

                feeds_list.push(FeedsList {
                    feed_url: url.to_string(),
                    webhook_url: webhook.unwrap().url().unwrap(),
                });

                db.set(
                    &guild_id,
                    &ServerData {
                        feeds_list: Some(feeds_list),
                        ..data
                    },
                )
                .unwrap();

                followup.content(format!("Subscribed to {url}"))
            } else {
                followup.content(
                    "Feed channel is not set. Please set feed updates channel with </set channel:1032703408399986841> and try again.",
                )
            }
        }
        Err(err) => {
            let reason = match err {
                GetFeedError::AccessError => "Cannot access to given URL.",
                GetFeedError::ParseError(parse_err) => match parse_err {
                    ParseFeedError::IoError(_) => "Unexpected error when parsing feed content. If this keeps happening, please report the issue to the developer.",
                    ParseFeedError::JsonSerde(_) => "Unexpected error when parsing feed content. If this keeps happening, please report the issue to the developer.",
                    ParseFeedError::JsonUnsupportedVersion(_) => "Unsupported JSON version on feed content.",
                    ParseFeedError::ParseError(_) => "Cannot parse given URL: Entered URL is not an RSS feed.",
                    ParseFeedError::XmlReader(_) => "Cannot read feed XML content: Looks like RSS content is broken."
                },
            };

            followup.content(format!("Cannot subscribe to <{url}>. Reason: {reason}",))
        }
    }
}

pub fn register() -> CreateApplicationCommand {
    CreateApplicationCommand::new("subscribe")
        .description("Subscribe to RSS feed.")
        .add_option(
            CreateApplicationCommandOption::new(
                CommandOptionType::String,
                "url",
                "URL of the RSS feed.",
            )
            .required(true),
        )
}
