use dotenvy;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
struct Handler;
static token = dotenvy::var("token").unwrap();
static client: Client = Client::builder(token, GatewayIntents::default()).event_handler(Handler).await?;
client.start().await()?;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&context, "Pong!");
        }
    }
}
