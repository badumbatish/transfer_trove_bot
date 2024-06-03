pub mod bot_message;

use poise::serenity_prelude::{self as serenity, FullEvent};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    println!("{} requested account creation date of {}", ctx.author().name, u.name);
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let option = poise::FrameworkOptions {
        commands: vec![age()],
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
    data: &Data,
) -> Result<(), Error> {

    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }

        serenity::FullEvent::Ratelimit { data } => {
            println!("Being rate limited");
        }

        serenity::FullEvent::Message { new_message } => {
            if new_message.author.id != ctx.cache.current_user().id {
                println!("{}", new_message.content.clone().to_string());

                if new_message.content.to_ascii_lowercase().contains("cal simp lords") {
                    println!("cal simp lords is mentioned");
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
