use dotenvy;
use poise::serenity_prelude as serenity;
use poise::framework::*;
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
/// Displays your or another user's account creation date/
static fw: Framework = Framework;
#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let shard = Framework.shard_manager(fw).runners.get(ctx.discord().shard_id);
    ctx.say(&format!("🏓**Pong!**\nping:  {:?}", shard.latency)).await?;
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
    fw = framework;
    framework.run().await.unwrap();
}
