use serenity::async_trait;
use serenity::model::{
    gateway::Ready,
    prelude::{command::Command, Interaction},
};
use serenity::prelude::{Context, EventHandler};

#[path = "commands/mod.rs"]
mod commands;

pub struct Events;
#[async_trait]
impl EventHandler for Events {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if command.data.name.as_str() == "latency" {
                commands::latency::run(&command.data.options(), &ctx, &command).await;
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let registered_commands = Command::get_global_application_commands(&ctx.http).await;
        let commands_to_register = vec![commands::latency::register()];

        if commands_to_register.len() < registered_commands.unwrap().len() {
            if let Err(why) =
                Command::set_global_application_commands(&ctx.http, commands_to_register).await
            {
                println!("Error when registering global commands: {}", why)
            }
        }

        println!("Connected to Discord as {}", ready.user.tag())
    }
}
