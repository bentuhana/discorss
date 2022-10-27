use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ResolvedOption;
use serenity::model::prelude::{CommandInteraction, ResolvedValue};
use serenity::model::webhook::Webhook;
use serenity::prelude::Context;

use crate::database::Database;
use crate::structs::feed::ServerData;

pub async fn run(
    options: &[ResolvedOption<'_>],
    ctx: &Context,
    interaction: &CommandInteraction,
) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();

    let mut db = Database::load(None);
    let guild_id = interaction.guild_id.unwrap().to_string();
    let ResolvedValue::SubCommand(_) = &options.get(0).unwrap().value else { return followup.content("Select a subcommand."); };

    let data = match db.get::<ServerData>(&guild_id) {
        Some(current_data) => {
            if current_data.feed_channel_id.is_none() && current_data.feed_webhook.is_none() {
                return followup.content("No channel set already.");
            }

            let current_webhook_data = current_data.feed_webhook.unwrap();
            if let Ok(current_webhook) = Webhook::from_id_with_token(
                &ctx.http,
                current_webhook_data.id,
                &current_webhook_data.token,
            )
            .await
            {
                if current_webhook.delete(&ctx.http).await.is_err() {
                    return followup.content("Could not delete webhook.");
                }
            } else {
                return followup.content("Could not get webhook.");
            }

            ServerData {
                feed_channel_id: None,
                feed_webhook: None,
                ..current_data
            }
        }
        None => return followup.content("No channel set already."),
    };

    db.set(&guild_id, &data).unwrap();
    followup.content("Unset channel.")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("unset")
        .description("Unset an option.")
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "channel",
            "Unset feed updates channel.",
        ))
}
