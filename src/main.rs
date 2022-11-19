use std::sync::Arc;
use dotenvy;
use poise::serenity_prelude as serenity;
use poise::framework::*;
use serenity::prelude::*;
use tokio::sync::Mutex;
use serenity::client::bridge::gateway::{ShardId, ShardManager};
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
// User data, which is stored and accessible in all command invocations
/// Displays your or another user's account creation date/

#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let data = ctx.discord().data.read().await;
    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            ctx.say("An error occurred");
            return Ok(());
        },
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;
    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            ctx.say("No shard found").await?;
            return Ok(());
        },
    };

    ctx.say(&format!("The shard latency is {:?}", runner.latency)).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn addcmd(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    //let token: String = dotenvy::var("token").unwrap();
    let fw = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), addcmd()],
            ..Default::default()
        })
        .token(dotenvy::var("token").unwrap())
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));
    fw.run().await.unwrap();
}
