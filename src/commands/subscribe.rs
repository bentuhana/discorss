use reqwest;
use url::{ParseError, Url};

use serenity::builder::{
    CreateApplicationCommand, CreateApplicationCommandOption, CreateInteractionResponseFollowup,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ResolvedOption, ResolvedValue};

pub async fn run(options: &[ResolvedOption<'_>]) -> CreateInteractionResponseFollowup {
    let followup = CreateInteractionResponseFollowup::new();

    let ResolvedValue::String(url) = &options.get(0).unwrap().value else { return followup.content("String value not found"); };

    match Url::parse(url) {
        Ok(url) => {
            if url.scheme() != "http" && url.scheme() != "https" {
                return followup.content("The URL entered must be using the HTTP(S) protocol.");
            }
        }
        Err(err) => {
            let reason = match err {
                ParseError::EmptyHost => "URL does not contain any host name.",
                ParseError::InvalidDomainCharacter => "URL have unsupported characters. Characters such as space are not allowed in URLs.",
                ParseError::InvalidIpv4Address => "Invalid IPv4 address.",
                ParseError::InvalidIpv6Address => "Invalid IPv6 address.",
                _ => "",
            };

            return followup.content(format!("Entered URL is not valid. {}", reason));
        }
    }

    if reqwest::get(url.to_string()).await.is_ok() {
        todo!("Implement subscribe command fully.");
    }

    followup.content("URL is valid.")

    // if let Ok(req) = reqwest::get(url.to_string()).await {
    //     let body = req.text().await.unwrap();

    //     if let Ok(parsed) = parser::parse(body.as_bytes()) {
    //         let prev_data =
    //             db.get::<ServerData>(interaction.guild_id.unwrap().to_string().as_str());

    //         let webhook_builder = CreateWebhook::new(parsed.title.unwrap().content).avatar(
    //             &CreateAttachment::url(&ctx.http, parsed.icon.unwrap().link.unwrap().href.as_str())
    //                 .await
    //                 .unwrap(),
    //         );
    //         let webhook = ChannelId(prev_data.unwrap().feed_channel_id.unwrap().parse().unwrap())
    //             .create_webhook(&ctx.http, webhook_builder)
    //             .await;

    //         let new_data: ServerData;
    //         if prev_data.is_some() {
    //             let mut feeds_list = prev_data.unwrap().feeds_list.unwrap();
    //             feeds_list.push(FeedsList {
    //                 feed_url: url.to_string(),
    //                 webhook_url: webhook.unwrap().url().unwrap(),
    //             });

    //             new_data = ServerData {
    //                 feeds_list: Some(feeds_list),
    //                 ..prev_data.unwrap()
    //             }
    //         } else {
    //             new_data = ServerData {
    //                 feeds_list: Some(vec![FeedsList {
    //                     feed_url: url.to_string(),
    //                     webhook_url: "TEST".to_string(),
    //                 }]),
    //                 ..Default::default()
    //             };
    //         }

    //         db.set(
    //             interaction.guild_id.unwrap().to_string().as_str(),
    //             &new_data,
    //         )
    //         .unwrap();
    //     } else {
    //         return followup.content("Cannot parse XML.");
    //     };
    // } else {
    //     return followup.content("Cannot access to URL.");
    // }
}

pub fn register() -> CreateApplicationCommand {
    CreateApplicationCommand::new("subscribe")
        .description("Subscribe to RSS feed.")
        .add_option(
            CreateApplicationCommandOption::new(
                CommandOptionType::String,
                "url",
                "URL of the RSS feed.",
            )
            .required(true),
        )
}
