use dotenvy;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use std::sync::Arc;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::TypeMapKey;
use poise::serenity_prelude::client::bridge::gateway::{ShardId, ShardManager};
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}
struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
pub trait AsyncRead {}
/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let data = ctx.data().read().await;
    let shard_manager = match data.get::<ShardManagerContainer>() {
      Some(v) => v,
      None => {
        ctx.say("Some problem occurred").await?;
        return Ok(());
      }
    };
    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;
    let runner = match runners.get(&ShardId(ctx.discord().shard_id)) {
      Some(runner) => runner,
      None => {
        ctx.say("Some problem occurred").await?;
        return Ok(());
      },
    };
    ctx.say(&format!("🏓**Pong!**\nping:  {:?}", runner.latency)).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn addcmd(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token: String = dotenvy::var("token").unwrap();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), addcmd()],
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}
