pub mod bot_message;
pub mod util;
pub mod commands;
use poise::serenity_prelude::{self as serenity, CacheHttp};
use util::PoiseUtil;
use commands::{Context, Data, Error};

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let option = poise::FrameworkOptions {
        commands: vec![commands::age(), commands::clean_up_channel_until_msg_id()],
        event_handler: |ctx, event, framework, data| {
                      Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(option)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {

    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }

        serenity::FullEvent::Ratelimit { data : _ } => {
            println!("Being rate limited");
        }

        serenity::FullEvent::Message { new_message } => {
            let msg = new_message.clone();
            PoiseUtil::get_channel_id(&msg);
            if new_message.author.id != ctx.cache.current_user().id {
                println!("{}", new_message.content.clone().to_string());

                if new_message.content.to_ascii_lowercase().contains("cal simp lords") {
                    let authorized = PoiseUtil::authorized(&ctx.http(), &msg).await;

                    if !authorized {
                        println!("Not authorized");
                        return Ok(());
                    }
                    new_message.channel_id.say(
                            ctx, bot_message::CAL_SIMP_TALE.to_string().as_str()
                        )
                        .await?;

                } else if new_message.content.to_ascii_lowercase().contains("poise") {
                    let msg = "errmmm, what are you doing around mentioning poise huhhhhh";
                    new_message.channel_id.say(
                            ctx, msg
                        )
                        .await?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
