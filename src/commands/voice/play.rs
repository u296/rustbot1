use super::prelude::*;

use serenity::async_trait;
use songbird::{EventContext, EventHandler};
use std::time::Duration;

use std::sync::Arc;

use songbird::Songbird;
use uuid::Uuid;

use serenity::model::prelude::*;

const IDLE_LEAVE_TIME: Duration = Duration::from_secs(10);

struct IdleLeaver {
    manager: Arc<Songbird>,
    typemap: Arc<RwLock<TypeMap>>,
    guild_id: GuildId,
    track_uuid: Uuid,
}

#[async_trait]
impl EventHandler for IdleLeaver {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<songbird::Event> {
        tokio::time::sleep(IDLE_LEAVE_TIME).await;

        // after waiting, the last played track still hasn't changed
        // which means that nothing has been played since
        if self
            .typemap
            .read()
            .await
            .get::<crate::utils::GuildDataMap>()
            .expect("no GuildDataMap in typemap")
            .get(&self.guild_id)
            .unwrap_or(&Default::default())
            .last_played_track
            == Some(self.track_uuid)
        {
            // it is not guaranteed that we are in the call anymore
            if let Some(call) = self.manager.get(self.guild_id) {
                let mut b = call.lock().await;

                match b.leave().await {
                    Ok(_) => (),
                    Err(e) => error!("{}", e),
                }
            }
        }

        None
    }
}

#[command]
#[aliases("p")]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let text = args.message();

    let source = {
        if text.starts_with("http") {
            debug!("source is a link");
            songbird::ytdl(text).await
        } else {
            debug!("source is search");
            songbird::input::ytdl_search(text).await
        }
    };

    play_backend(ctx, msg, args, source).await
}

#[command]
#[aliases("pl", "play local", "play saved")]
#[only_in(guilds)]
async fn play_local(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let source = {
        let filename = {
            let manifest =
                utils::ContentManifest::read_from_file(&utils::CONTENT_MANIFEST_PATH).await?;

            match manifest.uploads.get(args.message()) {
                Some(f) => f.clone(),
                None => {
                    msg.channel_id.say(ctx, "no such file").await?;
                    return Ok(());
                }
            }
        };

        let file: &str = &format!("content/{}", filename);
        debug!(file);

        songbird::ffmpeg(file).await
    };

    play_backend(ctx, msg, args, source).await
}

use songbird::input::{self, Input};

#[instrument(skip(ctx, msg, source))]
async fn play_backend(
    ctx: &Context,
    msg: &Message,
    args: Args,
    source: Result<Input, input::error::Error>,
) -> CommandResult {
    let text = args.message();
    let guild = msg.guild(ctx).await.unwrap();

    debug!(text, "{}", &guild.name);

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
        IdleLeaver {
            manager: songbird::get(ctx).await.unwrap(),
            typemap: ctx.data.clone(),
            guild_id: guild.id,
            track_uuid: trackhandle.uuid(),
        },
    )?;

    let mut typemap = ctx.data.write().await;

    let guild_data_map = typemap
        .get_mut::<utils::GuildDataMap>()
        .expect("no GuildDataMap in typemap")
        .entry(msg.guild_id.unwrap())
        .or_default();

    guild_data_map.tracks.push(trackhandle.clone());

    guild_data_map.last_played_track = Some(trackhandle.uuid());

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("shutup", "stfu")]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    for track in ctx
        .data
        .write()
        .await
        .get_mut::<utils::GuildDataMap>()
        .expect("no GuildDataMap in typemap")
        .entry(msg.guild_id.unwrap())
        .or_default()
        .tracks
        .drain(0..)
    {
        match track.stop() {
            Ok(_) => (),
            Err(e) => {
                error!("error stopping track: {}", e);
            }
        }
    }

    Ok(())
}
