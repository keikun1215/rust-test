use dotenvy;
static token: String = dotenvy::var("token").unwrap();
async fn main() {
    /*let options = poise::FrameworkOptions {
        commands: vec![register(), add()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("::".to_string()),
            ..Default::default()
        },
        on_error: |err| Box::pin(on_error(err)),
        ..Default::default()
    };*/

    poise::Framework::build()
        .token(token)
        //.options(options)
        .user_data_setup(|_, _, _| Box::pin(async { Ok(()) }))
        .run()
        .await
        .unwrap();
}
