use super::prelude::*;
use crate::wolframalpha;

#[command]
#[aliases("wfa")]
async fn wolframalpha(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    match ctx
        .data
        .read()
        .await
        .get::<wolframalpha::WolframalphaApikey>()
        .expect("no WolframalphaApikey in typemap")
    {
        Some(api_key) => {
            let gif_bytes = match wolframalpha_api::api_retrieve_bytes(api_key, args.message()).await? {
                Ok(b) => b,
                Err(_) => {
                    msg.channel_id.say(ctx, "invalid question").await?;
                    return Ok(())
                }
            };

            msg.channel_id
                .send_files(ctx, vec![(gif_bytes.as_ref(), "wfa.gif")], |m| m)
                .await?;
        }
        None => {
            msg.channel_id.say(ctx, "no api key available").await?;
        }
    }

    Ok(())
}
