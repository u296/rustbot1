use super::prelude::*;

#[command]
async fn start_timer(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    ctx.data
        .write()
        .await
        .get_mut::<utils::TextChannelDataMap>()
        .expect("no TextChannelDataMap in typemap")
        .entry(msg.channel_id)
        .or_default()
        .timer = Some(Instant::now());

    msg.channel_id.say(ctx, "timer started").await?;

    Ok(())
}

#[command]
async fn stop_timer(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let typemap = &mut ctx.data.write().await;

    let timer = &mut typemap
        .get_mut::<utils::TextChannelDataMap>()
        .expect("no TextChannelDataMaop in typemap")
        .entry(msg.channel_id)
        .or_default()
        .timer;

    match timer {
        Some(start) => {
            let duration: Duration = Instant::now() - start.clone();

            msg.channel_id
                .say(ctx, format!("{} s", duration.as_millis() as f32 / 1000.0))
                .await?;

            *timer = None;
        }
        None => {
            msg.channel_id.say(ctx, "no timer started").await?;
        }
    }

    Ok(())
}
