use std::error::Error;
use std::time::{Duration, Instant};

use serenity::model::prelude::*;


use serenity::prelude::*;
use futures::prelude::*;


pub async fn send_buffered<E, I>(ctx: &Context, channel: ChannelId, mut lines: I) -> Result<(), Box<dyn Error + Send + Sync>>
where E: Error, I: Unpin + Stream<Item = Result<String, E>> {

let mut output_buf = String::new();
let mut last_message_time = Instant::now();
while let Some(maybe_line) = lines.next().await {
    match maybe_line {
        Ok(line) => {
            output_buf.push_str(&format!("{}\n", String::from_utf8(strip_ansi_escapes::strip(&line)?)?));
            if !output_buf.trim().is_empty() && Instant::now() >= last_message_time + Duration::from_secs(2) {
                for chunk in output_buf.as_bytes().chunks(2000) {
                    channel.say(&ctx, std::str::from_utf8(chunk)?).await?;
                }
                last_message_time = Instant::now();
                output_buf = String::new();
            }
        },
        Err(e) => {
            println!("error: {}", e);
        }
    }
}
if !output_buf.trim().is_empty() {
    for chunk in output_buf.as_bytes().chunks(2000) {
        channel.say(&ctx, std::str::from_utf8(chunk)?).await?;
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
Ok(())
}