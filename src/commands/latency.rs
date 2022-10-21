use std::time::{Duration, Instant};

use serenity::builder::{
    CreateApplicationCommand, CreateInteractionResponse, CreateInteractionResponseFollowup,
    CreateInteractionResponseMessage,
};
use serenity::client::bridge::gateway::ShardId;
use serenity::model::prelude::interaction::application_command::ResolvedOption;
use serenity::model::prelude::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::structs::shard_manager::ShardManagerContainer;

pub async fn run(
    _options: &[ResolvedOption<'_>],
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) {
    let thinking_response_data = CreateInteractionResponseMessage::new();
    let thinking_response = CreateInteractionResponse::Defer(thinking_response_data);
    let followup = CreateInteractionResponseFollowup::new();

    let rest_latency_calculation_start = Instant::now();
    interaction
        .create_interaction_response(&ctx.http, thinking_response)
        .await
        .unwrap();
    let rest_latency = rest_latency_calculation_start.elapsed().as_millis();

    let mut message =
        format!("Pong! :ping_pong:\n\nREST latency: {rest_latency}ms\nGateway latency: ");

    let ctx_data = ctx.data.read().await;
    if let Some(shard_manager) = ctx_data.get::<ShardManagerContainer>() {
        let manager = shard_manager.lock().await;
        let runners = manager.runners.lock().await;

        if let Some(runner) = runners.get(&ShardId(ctx.shard_id)) {
            let gateway_latency = runner
                .latency
                .unwrap_or_else(|| Duration::from_millis(0))
                .as_millis();

            if gateway_latency > 0 {
                message.push_str(format!("{gateway_latency}ms").as_str());
            } else {
                message.push_str("No data avaliable at the moment.");
            }
        } else {
            message.push_str("No data avaliable at the moment.");
        }
    } else {
        message.push_str("No data avaliable at the moment.");
    }

    interaction
        .create_followup_message(&ctx.http, followup.content(message))
        .await
        .ok();
}

pub fn register() -> CreateApplicationCommand {
    CreateApplicationCommand::new("latency").description("Sends gateway and REST latency values.")
}
