use dotenvy;
use std::sync::Arc;
use tokio::sync::Mutex;
use poise::serenity_prelude as serenity;
use serenity::gateway::Shard;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let gateway = Arc::new(Mutex::new(http.get_gateway().await?.url));
    let png = gateway.latency().to_string();
    ctx.say(png).await?;
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
