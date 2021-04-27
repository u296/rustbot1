use std::error::Error;
use std::time::{Duration, Instant};

use serenity::constants::MESSAGE_CODE_LIMIT;
use serenity::http::AttachmentType;
use serenity::http::Http;
use serenity::model::prelude::*;

use futures::prelude::*;
use log::*;
use serenity::prelude::*;

fn get_latest_split_index(s: impl AsRef<str>, limit: usize) -> usize {
    for i in (0..limit).rev() {
        if s.as_ref().is_char_boundary(i) {
            return i;
        }
    }

    panic!("string not splittable");
}

fn split_line_to_sendable_chunks(line: impl AsRef<str>) -> Vec<String> {
    let mut chunks = vec![String::new()];

    let mut chunk_begin_index = 0;

    while chunk_begin_index != line.as_ref().len() {
        let chunk_end_index =
            get_latest_split_index(&line.as_ref()[chunk_begin_index..], MESSAGE_CODE_LIMIT);

        println!("chunk end index: {}", chunk_end_index);

        chunks.push(String::from(
            &line.as_ref()[chunk_begin_index..chunk_end_index],
        ));
        chunk_begin_index = chunk_end_index;
    }

    chunks
}

fn split_string_to_sendable_chunks(s: impl AsRef<str>) -> Vec<String> {
    let mut chunks = vec![String::new()];

    for line in s.as_ref().lines() {
        let last_chunk = chunks.last_mut().unwrap();

        if !(last_chunk.len() + line.len() + 1 > MESSAGE_CODE_LIMIT) {
            last_chunk.push('\n');
            last_chunk.push_str(line);
        } else {
            chunks.extend(split_line_to_sendable_chunks(line))
        }
    }

    chunks
}

fn time_has_passed(passed: Duration, last_time: &mut Instant) -> bool {
    if Instant::now() >= last_time.clone() + passed {
        *last_time = Instant::now();
        true
    } else {
        false
    }
}

pub async fn send_buffered_text<S, I>(
    ctx: &Context,
    channel: ChannelId,
    mut lines: I,
) -> Result<(), Box<dyn Error + Send + Sync>>
where
    S: AsRef<str>,
    I: Unpin + Stream<Item = S>,
{
    let mut send_chunks = Vec::new();
    let mut output_buf = String::new();
    let mut last_send_time = Instant::now();

    while let Some(line) = lines.next().await {
        let line = String::from_utf8(strip_ansi_escapes::strip(line.as_ref())?)?;

        output_buf.push_str(&format!("\n{}", line));

        if time_has_passed(Duration::from_secs(1), &mut last_send_time) {
            send_chunks.extend(split_string_to_sendable_chunks(output_buf));
            output_buf = String::new();

            // loop here so we actually send something
            while !send_chunks.is_empty() {
                let chunk = send_chunks.remove(0);
                if !chunk.trim().is_empty() {
                    channel.say(&ctx, chunk).await?;
                    break;
                }
            }
        }
    }
    send_chunks.extend(split_string_to_sendable_chunks(output_buf));
    for chunk in send_chunks {
        if !chunk.trim().is_empty() {
            channel.say(&ctx, chunk).await?;
        }
    }

    Ok(())
}

pub async fn send_text_file<'a, P: Into<AttachmentType<'a>>, I: Iterator<Item = P>>(
    ctx: &Context,
    channel: ChannelId,
    files: I,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    channel.send_files(ctx, files, |e| e).await?;

    Ok(())
}

pub async fn repeat_mention<M: Mentionable>(
    http: &impl AsRef<Http>,
    channel: ChannelId,
    mention: &M,
    count: usize,
    delay: Duration,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    for _ in 0..count {
        channel.say(http, format!("{}", mention.mention())).await?;

        tokio::time::sleep(delay).await;
    }
    Ok(())
}

#[test]
fn test_get_latest_split_index() {
    assert_eq!(get_latest_split_index("s: impl AsRef<str>", 5), 4);
    assert_eq!(get_latest_split_index("bruh momento", 12), 11);
    assert_eq!(get_latest_split_index("bruh 刻", 8), 5);
}
