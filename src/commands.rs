use poise::serenity_prelude::{self as serenity, CacheHttp, ChannelId, MessageId};
use serenity::builder::GetMessages;



pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    println!("{} requested account creation date of {}", ctx.author().name, u.name);
    Ok(())
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn clean_up_channel_until_msg_id(
    ctx: Context<'_>,
    #[description = "Selected message id"] message_id: Option<serenity::MessageId>,
) -> Result<(), Error> {
    println!("Performing clean_up_channel_until_msg_id");
    let authorized = ctx.author().has_role(ctx.http(), ctx.guild_id().unwrap(), crate::util::ADMIN_ROLE_ID).await?;
    if !authorized {
        ctx.say("You are not authorized to use this feature").await?;
        return Ok(());
    }

    let channel_id = ctx.channel_id();

    match message_id {
        Some(m_id) => {
            let builder = GetMessages::new().after(m_id).limit(100);
            let messages = channel_id.messages(ctx.http(), builder).await?;
            for message in messages.iter() {
                    message.delete(ctx.http()).await?;
                }
            ctx.say("Deletion completed").await?;
            ()
        }
        _ => {
            ctx.say("Deletion unsucessful").await?;
            ()
        }
    }

    Ok(())
}
