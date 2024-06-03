use poise::serenity_prelude::{ChannelId, Http, Message};



pub const ADMIN_BOT_CHANNEL_ID : u64 = 1246962917744574606;

pub const JJASMINE_LOG_CHANNEL_ID : u64 = 1246963053015335044;

pub const ADMIN_ROLE_ID : u64 = 1246931217434017852;

pub const NORMIE_ROLE_ID : u64 = 1247016774743097425;

pub struct PoiseUtil {}

impl PoiseUtil {
    pub fn get_channel_id(msg : &Message) -> ChannelId {
        return msg.channel_id;
    }

    pub async fn authorized(h: &Http, msg : &Message) -> bool {
        let guild_id = msg.guild_id;

        if Self::get_channel_id(&msg) == ADMIN_BOT_CHANNEL_ID || Self::get_channel_id(&msg) == JJASMINE_LOG_CHANNEL_ID {
            return true;
        } else {
            match guild_id {
                Some(id) => msg.author.has_role(h, id, ADMIN_ROLE_ID).await.unwrap_or(false),
                None => false
            }
        }
    }
}
