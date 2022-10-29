use serenity::builder::{CreateCommand, CreateInteractionResponseFollowup};
use serenity::model::prelude::interaction::application_command::ResolvedOption;
use serenity::model::prelude::CommandInteraction;

use crate::database;
use crate::structs::feed::ServerData;

pub fn run(
    _options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();

    let db = database::load(None);
    let guild_id = interaction.guild_id.unwrap().to_string();
    let subscriptions = match db.get::<ServerData>(&guild_id) {
        Some(data) => data.feeds_list.unwrap_or_default(),
        None => vec![],
    };

    if subscriptions.is_empty() {
        followup.content("No subscription.")
    } else {
        let mut codeblock = "```md\n".to_string();

        let mut index = 0;
        for subscription in subscriptions {
            index += 1;
            codeblock.push_str(format!("{index}. {subscription}\n").as_str());
        }
        codeblock.push_str("```");

        followup.content(codeblock)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("subscriptions").description("List current subscriptions")
}
