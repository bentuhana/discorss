use serenity::async_trait;
use serenity::builder::{
    CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage,
};
use serenity::model::gateway::Ready;
use serenity::model::prelude::{command::Command, Interaction};
use serenity::prelude::{Context, EventHandler};

use crate::commands;

pub struct Events;
#[async_trait]
impl EventHandler for Events {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                // Latency command is an exception and we are
                // returning since we are calculating REST latency
                // on commands/latency.rs#L23-L38
                "latency" => {
                    commands::latency::run(&command.data.options(), &ctx, &command).await;
                    return;
                }
                "set" => commands::set::channel::run(&command.data.options(), &ctx, &command).await,
                "unset" => {
                    commands::unset::channel::run(&command.data.options(), &ctx, &command).await
                }
                "subscribe" => commands::subscribe::run(&command.data.options(), &command).await,
                "unsubscribe" => {
                    commands::unsubscribe::run(&command.data.options(), &command).await
                }
                cmd => CreateInteractionResponseFollowup::new()
                    .content(format!("No such command found: {cmd}")),
            };

            let thinking_response_data = CreateInteractionResponseMessage::new();
            let thinking_response = CreateInteractionResponse::Defer(thinking_response_data);

            if let Err(why) = command
                .create_interaction_response(&ctx.http, thinking_response)
                .await
            {
                warn!(
                    "Cannot create thinking instance on command {}: {why}",
                    command.data.name
                )
            }
            if let Err(why) = command.create_followup_message(&ctx.http, content).await {
                warn!(
                    "Cannot respond to thinking instance on command {}: {why}",
                    command.data.name
                )
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let registered_commands = Command::get_global_application_commands(&ctx.http).await;
        let commands_to_register = vec![
            commands::latency::register(),
            commands::set::channel::register(),
            commands::unset::channel::register(),
            commands::subscribe::register(),
            commands::unsubscribe::register(),
        ];

        if commands_to_register.len() >= registered_commands.unwrap().len() {
            if let Err(why) =
                Command::set_global_application_commands(&ctx.http, commands_to_register).await
            {
                error!("Error when registering global commands: {why}")
            }
        }

        info!("Connected to Discord as {}!", ready.user.tag())
    }
}
