use dotenvy;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ShardId;
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Send gateway latency
#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let shmp = &*ctx.framework().shard_manager; //Get shard manager
    let s1 = &*shmp.lock().await;
    let s2 = s1.runners.lock().await; //Get shard runners
    let runner = s2.get(&ShardId(ctx.discord().shard_id)).unwrap(); //Get runner
    let ping = runner.latency.unwrap(); //Get runner latency
    ctx.say(&format!("🏓**Pong!**\nping: {:?}", ping)).await?; //Send runner latency
    Ok(())
}

/// Send server info
#[poise::command(slash_command, prefix_command)]
async fn svrinfo(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let mut b_or_u = vec![];
    let blevel = match &ctx.guild().unwrap().premium_tier {
      Tier0 => "0",
      Tier1 => "1",
      Tier2 => "2",
      Tier3 => "3",
      Unknown => "?"
    };
    for (k, v) in &ctx.guild().unwrap().members {
      &b_or_u.push(v.user.bot);
    }
    let mut bou2 = &b_or_u;
    ctx.send(|cr| {
      cr.embed(|CreateEmbed| {
        CreateEmbed
          .title("Server information")
          .field("Members", format!("**👥Total**: {}\n**🤖Bots**: {}\n**👤Users**: {}", bou2.len(), bou2.into_iter().filter(|b| **b).count(), bou2.into_iter().filter(|b| !**b).count()), true)
          .field("Server", format!("**🌏Region**: {}\n<:boost:1043830452655501402>**Boost**: `{}`\n<:id:1043845860175392768>**ID**: {}", &ctx.guild().unwrap().preferred_locale, blevel, &ctx.guild().unwrap().id.0), true)
      })
    }).await?;
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
            commands: vec![ping(), svrinfo(), addcmd()],
            ..Default::default()
        })
        .token(dotenvy::var("token").unwrap())
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));
    fw.run().await.unwrap();
}
