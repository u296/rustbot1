use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::id::*;
use serenity::prelude::*;

use songbird::EventContext;
use songbird::Songbird;

use tracing::*;

use crate::utils;

// fisrt string is what to react to, second is reaction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Response {
    AudioCue((String, String)),
    TextReply((String, String)),
}

struct Leaver {
    manager: Arc<Songbird>,
    guild_id: GuildId,
}

#[async_trait]
impl songbird::EventHandler for Leaver {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<songbird::Event> {
        if let Some(call) = self.manager.get(self.guild_id) {
            let mut c = call.lock().await;
            match c.leave().await {
                Ok(_) => (),
                Err(e) => error!("{}", e),
            }
        }

        None
    }
}

impl Response {
    pub fn get_trigger(&self) -> &str {
        match self {
            Self::AudioCue((s, _)) => &s,
            Self::TextReply((s, _)) => &s,
        }
    }

    pub async fn exec(
        &self,
        ctx: &Context,
        msg: &Message,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if msg.content.contains(self.get_trigger()) {
            match self {
                Self::AudioCue((_, answer)) => {
                    let source = {
                        let filename = {
                            let manifest = crate::utils::ContentManifest::read_from_file(
                                &crate::utils::CONTENT_MANIFEST_PATH,
                            )
                            .await?;

                            match manifest.uploads.get(answer) {
                                Some(f) => f.clone(),
                                None => {
                                    msg.channel_id.say(ctx, "no such file").await?;
                                    return Ok(());
                                }
                            }
                        };

                        let file: &str = &format!("content/{}", filename);

                        songbird::ffmpeg(file).await
                    };

                    let guild = msg.guild(ctx).await.unwrap();

                    let source = match source {
                        Ok(src) => src,
                        Err(e) => {
                            msg.channel_id
                                .say(ctx, format!("error starting source: {:?}", e))
                                .await?;
                            return Err(format!("{:?}", e).into());
                        }
                    };

                    let maybe_vc = utils::get_user_voice_channel(&guild, &msg.author);

                    let call = if let Some(vc) = maybe_vc {
                        utils::join_voice_channel(ctx, &guild, &vc).await?
                    } else {
                        msg.channel_id
                            .say(ctx, "you are not in a voice channel")
                            .await?;
                        return Ok(());
                    };

                    let trackhandle = utils::play_from_input(call, source).await;

                    trackhandle.add_event(
                        songbird::Event::Track(songbird::TrackEvent::End),
                        Leaver {
                            manager: songbird::get(ctx).await.unwrap(),
                            guild_id: guild.id,
                        },
                    )?;

                    Ok(())
                }

                Self::TextReply((_, answer)) => {
                    msg.channel_id.say(ctx, answer).await?;
                    Ok(())
                },
            }
        } else {
            Ok(())
        }
    }
}
