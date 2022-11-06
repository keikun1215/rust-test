use dotenv::dotenv;
use std::env;
use std::process;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
struct Handler;
dotenv().ok();
let token = match env::var("token") {
  Ok(val) => val
};
let mut client = Client::builder(token, GatewayIntents::default()).event_handler(Handler).await?;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&context, "Pong!");
        }
    }
}

client.start().await()?;
