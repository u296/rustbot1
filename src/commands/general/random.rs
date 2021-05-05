use super::prelude::*;
use rand::seq::SliceRandom;

#[command]
#[aliases("pick", "random")]
async fn select_random(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let names: Vec<String> = args.iter::<String>().filter_map(|m| m.ok()).collect();

    let s: &str = match names.choose(&mut rand::thread_rng()) {
        Some(b) => &b,
        None => "",
    };

    if !s.trim().is_empty() {
        msg.channel_id.say(ctx, s).await?;
    }

    Ok(())
}
