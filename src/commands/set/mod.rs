use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ResolvedOption, ResolvedValue};
use serenity::model::prelude::CommandInteraction;
use serenity::prelude::Context;

mod channel;
mod set_interval;

pub async fn run(
    options: &[ResolvedOption<'_>],
    ctx: &Context,
    interaction: &CommandInteraction,
) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();

    let sub_command = &options.first().unwrap();
    let ResolvedValue::SubCommand(sub_command_value) = &sub_command.value else { return followup.content("Enter subcommand.") };

    if sub_command.name == "channel" {
        let ResolvedValue::Channel(channel) = sub_command_value.first().unwrap().value else { return followup.content("Mention a channel to set."); };
        channel::run(&[channel.to_owned()], ctx, interaction).await
    } else if sub_command.name == "interval" {
        let ResolvedValue::Integer(minutes) = sub_command_value.first().unwrap().value else { return followup.content("Mention a channel to set."); };
        set_interval::run(&[minutes.to_owned()], interaction)
    } else {
        followup.content("Sub command not found.")
    }
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
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "interval", "Test")
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::Integer, "hours", "hours")
                        .min_int_value(1)
                        .max_int_value(24)
                        .required(true),
                ),
        )
}
