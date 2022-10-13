use std::time::{Duration, Instant};

use serenity::builder::{
    CreateApplicationCommand, CreateInteractionResponse, CreateInteractionResponseData,
    CreateInteractionResponseFollowup,
};
use serenity::client::bridge::gateway::ShardId;
use serenity::model::prelude::interaction::application_command::ResolvedOption;
use serenity::model::prelude::{ApplicationCommandInteraction, InteractionResponseType};
use serenity::prelude::Context;

use crate::shard_manager::ShardManagerContainer;

pub async fn run(
    _options: &[ResolvedOption<'_>],
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) {
    let thinking_response_data = CreateInteractionResponseData::new();
    let thinking_response = CreateInteractionResponse::new()
        .kind(InteractionResponseType::DeferredChannelMessageWithSource)
        .interaction_response_data(thinking_response_data);
    let mut followup = CreateInteractionResponseFollowup::new();

    let rest_latency_calculation_start = Instant::now();
    interaction
        .create_interaction_response(&ctx.http, thinking_response)
        .await
        .ok();
    let rest_latency = rest_latency_calculation_start.elapsed().as_millis();

    let ctx_data = ctx.data.read().await;
    let shard_manager = match ctx_data.get::<ShardManagerContainer>() {
        Some(shard) => shard,
        None => {
            followup = followup.content("Couldn't get ShardManager.".to_string());
            interaction
                .create_followup_message(&ctx.http, followup)
                .await
                .ok();

            return;
        }
    };
    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            followup = followup.content("Couldn't find any shard.".to_string());
            interaction
                .create_followup_message(&ctx.http, followup)
                .await
                .ok();

            return;
        }
    };
    let gateway_latency = runner
        .latency
        .unwrap_or_else(|| Duration::from_millis(0))
        .as_millis();

    followup = followup.content(format!(
        "Pong! :ping_pong:\n\nREST latency: {}ms\nGateway latency: {}",
        rest_latency,
        if gateway_latency > 0 {
            format!("{}ms", gateway_latency)
        } else {
            "No data available at the moment.".to_string()
        }
    ));

    interaction
        .create_followup_message(&ctx.http, followup)
        .await
        .ok();
}

pub fn register() -> CreateApplicationCommand {
    CreateApplicationCommand::new("latency").description("Sends gateway and REST latency values.")
}
