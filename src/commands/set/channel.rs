use serenity::builder::{
    CreateApplicationCommand, CreateApplicationCommandOption, CreateInteractionResponseFollowup,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ResolvedOption, ResolvedValue};
use serenity::model::prelude::ChannelType;

pub fn run(options: &[ResolvedOption]) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();

    let ResolvedValue::SubCommand(sub_command) = &options.get(0).unwrap().value else { return followup.content("ASD"); };
    let ResolvedValue::Channel(channel) = sub_command.get(0).unwrap().value else { return followup.content("ASD"); };

    if channel.kind != ChannelType::Text {
        return followup.content("Channel must be a text channel.");
    }

    followup.content(format!("{}", channel.id))
}

pub fn register() -> CreateApplicationCommand {
    CreateApplicationCommand::new("set")
        .description("Sends gateway and REST latency values.")
        .add_option(
            CreateApplicationCommandOption::new(
                CommandOptionType::SubCommand,
                "channel",
                "Set feed updates channel.",
            )
            .add_sub_option(
                CreateApplicationCommandOption::new(
                    CommandOptionType::Channel,
                    "channel",
                    "Channel to send feed updates.",
                )
                .required(true),
            ),
        )
}
