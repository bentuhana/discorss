use serenity::builder::{
    CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup, CreateWebhook,
    EditWebhook,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ResolvedOption, ResolvedValue};
use serenity::model::prelude::{ChannelType, CommandInteraction};
use serenity::model::webhook::Webhook;
use serenity::prelude::Context;

use crate::database::Database;
use crate::structs::feed::{FeedWebhook, ServerData};

pub async fn run(
    options: &[ResolvedOption<'_>],
    ctx: &Context,
    interaction: &CommandInteraction,
) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();

    let mut db = Database::load(None);
    let guild_id = interaction.guild_id.unwrap().to_string();
    let ResolvedValue::SubCommand(sub_command) = &options.get(0).unwrap().value else { return followup.content("Select a subcommand."); };
    let ResolvedValue::Channel(channel) = sub_command.get(0).unwrap().value else { return followup.content("Mention a channel to set."); };

    if channel.kind != ChannelType::Text {
        return followup.content("Mentioned channel must be a text channel.");
    }

    let data: ServerData;
    if let Some(current_data) = db.get::<ServerData>(&guild_id) {
        // TODO: use Option::is_some_and() when it lands to Rust.
        // Tracking issue: https://github.com/rust-lang/rust/issues/93050
        if matches!(current_data.feed_channel_id, Some(id) if id == channel.id) {
            return followup.content(format!(
                "Feed updates channel is already set to <#{}>.",
                channel.id
            ));
        }

        let webhook: Webhook;
        if current_data.feed_webhook.is_some() {
            let current_webhook = current_data.feed_webhook.unwrap();
            // TODO: fix editing Webhook not working.
            if let Ok(edited_webhook) = EditWebhook::new()
                .channel_id(channel.id)
                .execute(&ctx.http, current_webhook.id, Some(&current_webhook.token))
                .await
            {
                webhook = edited_webhook;
            } else {
                return followup.content("Could not edit existing webhook.");
            }
        } else if let Ok(new_webhook) = CreateWebhook::new("DiscoRSS Feed")
            .execute(&ctx.http, channel.id)
            .await
        {
            webhook = new_webhook
        } else {
            return followup.content("Could not create new webhook.");
        }

        data = ServerData {
            feed_channel_id: Some(channel.id),
            feed_webhook: Some(FeedWebhook {
                id: webhook.id,
                token: webhook.token.unwrap(),
            }),
            ..current_data
        }
    } else if let Ok(new_webhook) = CreateWebhook::new("DiscoRSS Feed")
        .execute(&ctx.http, channel.id)
        .await
    {
        data = ServerData {
            feed_channel_id: Some(channel.id),
            feed_webhook: Some(FeedWebhook {
                id: new_webhook.id,
                token: new_webhook.token.unwrap(),
            }),
            ..Default::default()
        }
    } else {
        return followup.content("Could not create new webhook.");
    }

    db.set(&guild_id, &data).unwrap();
    followup.content(format!("Feed updates channel is set to <#{}>.", channel.id))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("set")
        .description("Set an option.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "channel",
                "Set feed updates channel.",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::Channel,
                    "channel",
                    "Channel to send feed updates.",
                )
                .required(true),
            ),
        )
}
