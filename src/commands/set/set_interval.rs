use serenity::builder::CreateInteractionResponseFollowup;
use serenity::model::prelude::CommandInteraction;

use crate::database;
use crate::structs::feed::ServerData;

pub fn run(options: &[i64], interaction: &CommandInteraction) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();

    let mut db = database::load(None);
    let guild_id = interaction.guild_id.unwrap().to_string();

    let minutes = options.first().unwrap();

    let data = match db.get::<ServerData>(&guild_id) {
        Some(current_data) => ServerData {
            feed_check_interval: Some(minutes * 60 * 60),
            ..current_data
        },
        None => return followup.content("Interval left default"),
    };

    db.set(&guild_id, &data).unwrap();
    followup.content("Set interval.")
}
