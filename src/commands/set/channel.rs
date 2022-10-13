use serenity::builder::{
    CreateApplicationCommand, CreateApplicationCommandOption, CreateInteractionResponseFollowup,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ResolvedOption, ResolvedValue};
use serenity::model::prelude::{ApplicationCommandInteraction, ChannelType};

#[path = "../../database.rs"]
mod database;
use database::{Database, ServerData};

pub fn run(
    options: &[ResolvedOption],
    interaction: &ApplicationCommandInteraction,
) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();
    let mut db = Database::new(Some(pickledb::PickleDbDumpPolicy::AutoDump));

    let ResolvedValue::SubCommand(sub_command) = &options.get(0).unwrap().value else { return followup.content("Couldn't get SubCommand value."); };
    let ResolvedValue::Channel(channel) = sub_command.get(0).unwrap().value else { return followup.content("Couldn't get Channel value."); };

    if channel.kind != ChannelType::Text {
        return followup.content("Channel must be a text channel.");
    }

    db.set(
        interaction.guild_id.unwrap().to_string().as_str(),
        &ServerData {
            feed_channel_id: Some(channel.id),
            ..Default::default()
        },
    )
    .unwrap();

    followup.content(format!("Feed updates channel is set to <#{}>.", channel.id))
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
