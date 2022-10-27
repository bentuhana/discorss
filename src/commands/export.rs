use opml::OPML;
use url::Url;

use serenity::builder::{CreateAttachment, CreateCommand, CreateInteractionResponseFollowup};
use serenity::model::prelude::interaction::application_command::ResolvedOption;
use serenity::model::prelude::CommandInteraction;

use crate::database::Database;
use crate::structs::feed::ServerData;

pub fn run(
    _options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();

    let db = Database::load(None);
    let guild_id = interaction.guild_id.unwrap().to_string();

    let attachment = match db.get::<ServerData>(&guild_id) {
        Some(current_data) => {
            let mut opml = OPML::default();
            let current_feeds_list = current_data.feeds_list;
            if current_feeds_list.is_none() {
                return followup.content("No subscription.");
            }

            for feed in current_feeds_list.unwrap() {
                opml.add_feed(Url::parse(&feed).unwrap().host_str().unwrap(), &feed);
            }

            CreateAttachment::bytes(opml.to_string().unwrap().as_bytes(), "DiscoRSS_Export.OPML")
        }
        None => return followup.content("No subscription."),
    };

    followup.add_file(attachment)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("export").description("Export feeds list as an OPML file.")
}
