use serenity::async_trait;
use serenity::builder::{
    CreateInteractionResponse, CreateInteractionResponseData, CreateInteractionResponseFollowup,
};
use serenity::model::{
    gateway::Ready,
    prelude::{command::Command, Interaction, InteractionResponseType},
};
use serenity::prelude::{Context, EventHandler};

#[path = "commands/mod.rs"]
mod commands;

pub struct Events;
#[async_trait]
impl EventHandler for Events {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                // Latency command is an exception and we are returning since we are calculating REST latency on
                // commands/latency.rs#L25-L30
                "latency" => {
                    commands::latency::run(&command.data.options(), &ctx, &command).await;
                    return;
                }
                "set" => commands::set::channel::run(&command.data.options(), &command),
                cmd => CreateInteractionResponseFollowup::new()
                    .content(format!("No such command found: {}", cmd)),
            };

            let thinking_response_data = CreateInteractionResponseData::new();
            let thinking_response = CreateInteractionResponse::new()
                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(thinking_response_data);

            if let Err(why) = command
                .create_interaction_response(&ctx.http, thinking_response)
                .await
            {
                println!(
                    "Cannot create thinking instance on command {}: {}",
                    command.data.name, why
                )
            }
            if let Err(why) = command.create_followup_message(&ctx.http, content).await {
                println!(
                    "Cannot followup thinking instance on command {}: {}",
                    command.data.name, why
                )
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let registered_commands = Command::get_global_application_commands(&ctx.http).await;
        let commands_to_register = vec![
            commands::latency::register(),
            commands::set::channel::register(),
        ];

        if commands_to_register.len() >= registered_commands.unwrap().len() {
            if let Err(why) =
                Command::set_global_application_commands(&ctx.http, commands_to_register).await
            {
                println!("Error when registering global commands: {}", why)
            }
        }

        println!("Connected to Discord as {}", ready.user.tag())
    }
}
