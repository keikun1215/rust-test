use std::sync::Arc;
use dotenvy;
use poise::serenity_prelude as serenity;
use tokio::sync::Mutex;
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
/// Displays your or another user's account creation date/

#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let shmp = *ctx.framework().shard_manager;
    let runner = *shmp.lock().await.runners.lock().unwrap().get(ctx.discord().shard_id);;
    //let runner = shmp2
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
