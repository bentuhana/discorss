use std::time::{Duration, Instant};

use serenity::builder::{
    CreateCommand, CreateInteractionResponse, CreateInteractionResponseFollowup,
    CreateInteractionResponseMessage,
};
use serenity::client::bridge::gateway::ShardId;
use serenity::model::prelude::interaction::application_command::ResolvedOption;
use serenity::model::prelude::CommandInteraction;
use serenity::prelude::Context;

use crate::structs::shard_manager::ShardManagerContainer;

pub async fn run(_options: &[ResolvedOption<'_>], ctx: &Context, interaction: &CommandInteraction) {
    let thinking_response_data = CreateInteractionResponseMessage::new();
    let thinking_response = CreateInteractionResponse::Defer(thinking_response_data);
    let followup = CreateInteractionResponseFollowup::new();

    let rest_latency_calculation_start = Instant::now();
    interaction
        .create_response(&ctx.http, thinking_response)
        .await
        .unwrap();
    let rest_latency = rest_latency_calculation_start.elapsed().as_millis();

    let ctx_data = ctx.data.read().await;
    let message_end = match ctx_data.get::<ShardManagerContainer>() {
        Some(shard_manager) => {
            let manager = shard_manager.lock().await;
            let runners = manager.runners.lock().await;

            if let Some(runner) = runners.get(&ShardId(ctx.shard_id)) {
                let gateway_latency = runner
                    .latency
                    .unwrap_or_else(|| Duration::from_millis(0))
                    .as_millis();

                if gateway_latency > 0 {
                    format!("{gateway_latency}ms")
                } else {
                    "No data avaliable at the moment.".to_string()
                }
            } else {
                "No data avaliable at the moment.".to_string()
            }
        }
        None => "No data avaliable at the moment.".to_string(),
    };

    interaction
        .create_followup(
            &ctx.http,
            followup.content(format!(
            "Pong! :ping_pong:\n\nREST latency: {rest_latency}ms\nGateway latency: {message_end}"
        )),
        )
        .await
        .ok();
}

pub fn register() -> CreateCommand {
    CreateCommand::new("latency").description("Sends gateway and REST latency values.")
}
