use serenity::builder::{
    CreateApplicationCommand, CreateApplicationCommandOption, CreateInteractionResponseFollowup,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ResolvedOption, ResolvedValue};
use serenity::model::prelude::{ApplicationCommandInteraction, ChannelType};

use crate::database::Database;
use crate::structs::database::ServerData;

pub fn run(
    options: &[ResolvedOption],
    interaction: &ApplicationCommandInteraction,
) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();
    let guild_id = interaction.guild_id.unwrap().to_string();
    let mut db = Database::load(None);

    let ResolvedValue::SubCommand(sub_command) = &options.get(0).unwrap().value else { return followup.content("Select a subcommand."); };
    let ResolvedValue::Channel(channel) = sub_command.get(0).unwrap().value else { return followup.content("Mention a channel to set."); };

    if channel.kind != ChannelType::Text {
        return followup.content("Mentioned channel must be a text channel.");
    }

    let data: ServerData;
    if let Some(prev_data) = db.get::<ServerData>(&guild_id) {
        data = ServerData {
            feed_channel_id: Some(channel.id.to_string()),
            ..prev_data
        };
    } else {
        data = ServerData {
            feed_channel_id: Some(channel.id.to_string()),
            ..Default::default()
        };
    }

    db.set(&guild_id, &data).unwrap();

    followup.content(format!("Feed updates channel is set to <#{}>.", channel.id))
}

pub fn register() -> CreateApplicationCommand {
    CreateApplicationCommand::new("set")
        .description("Set an option.")
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
