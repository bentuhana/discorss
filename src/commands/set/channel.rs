use serenity::builder::{CreateInteractionResponseFollowup, CreateWebhook, EditWebhook};
use serenity::model::prelude::{ChannelType, CommandInteraction, PartialChannel};
use serenity::model::webhook::Webhook;
use serenity::prelude::Context;

use crate::database;
use crate::structs::feed::{FeedWebhook, ServerData};

pub async fn run(
    options: &[PartialChannel],
    ctx: &Context,
    interaction: &CommandInteraction,
) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();

    let mut db = database::load(None);
    let guild_id = interaction.guild_id.unwrap().to_string();

    let channel = options.get(0).unwrap();

    if channel.kind != ChannelType::Text {
        return followup.content("Mentioned channel must be a text channel.");
    }

    let data = match db.get::<ServerData>(&guild_id) {
        Some(current_data) => {
            // TODO: use Option::is_some_and() when it lands to Rust.
            // * Tracking issue: https://github.com/rust-lang/rust/issues/93050
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
                // ! This might be serenity's issue.
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

            ServerData {
                feed_channel_id: Some(channel.id),
                feed_webhook: Some(FeedWebhook {
                    id: webhook.id,
                    token: webhook.token.unwrap(),
                }),
                ..current_data
            }
        }
        None => {
            if let Ok(new_webhook) = CreateWebhook::new("DiscoRSS Feed")
                .execute(&ctx.http, channel.id)
                .await
            {
                ServerData {
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
        }
    };

    db.set(&guild_id, &data).unwrap();
    followup.content(format!("Feed updates channel is set to <#{}>.", channel.id))
}
