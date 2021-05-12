use super::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};
use std::ops::Deref;

use serenity::constants::MESSAGE_CODE_LIMIT;
use serenity::http::AttachmentType;
use serenity::http::Http;
use serenity::model::prelude::*;

use futures::prelude::*;
use serenity::prelude::*;

#[derive(Default, Clone, Copy, Debug)]
pub struct TextChannelData {
    pub timer: Option<Instant>,
}

#[derive(Default, Clone, Debug)]
pub struct TextChannelDataMap(pub HashMap<ChannelId, TextChannelData>);

impl TextChannelDataMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Deref for TextChannelDataMap {
    type Target = HashMap<ChannelId, TextChannelData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TextChannelDataMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeMapKey for TextChannelDataMap {
    type Value = Self;
}

#[instrument(skip(s))]
fn get_latest_split_index(s: impl AsRef<str>, limit: usize) -> usize {
    trace!("s: {}", s.as_ref());

    for i in (0..limit).rev() {
        if s.as_ref().is_char_boundary(i) {
            trace!("index: {}", i);
            return i;
        }
    }

    panic!("string not splittable");
}

#[instrument(skip(line))]
fn split_line_to_sendable_chunks(line: impl AsRef<str>) -> Vec<String> {
    trace!("line: {}", line.as_ref());

    let mut chunks = vec![String::new()];

    let mut chunk_begin_index = 0;

    while chunk_begin_index != line.as_ref().len() {
        let chunk_end_index =
            get_latest_split_index(&line.as_ref()[chunk_begin_index..], MESSAGE_CODE_LIMIT);

        chunks.push(String::from(
            &line.as_ref()[chunk_begin_index..chunk_end_index],
        ));
        chunk_begin_index = chunk_end_index;
    }

    chunks
}

#[instrument(skip(s))]
fn split_string_to_sendable_chunks(s: impl AsRef<str>) -> Vec<String> {
    trace!("s: {}", s.as_ref());

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

    trace!("chunks.len: {}", chunks.len());

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

#[instrument(skip(ctx, lines))]
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
        trace!("received new line: {}", line);

        output_buf.push_str(&format!("\n{}", line));

        if time_has_passed(Duration::from_secs(1), &mut last_send_time) {
            send_chunks.extend(split_string_to_sendable_chunks(output_buf));
            output_buf = String::new();

            debug!("sending {} chunks", send_chunks.len());
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
    debug!("stream empty, remaining chunks: {}", send_chunks.len());
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

#[instrument(skip(http, mention))]
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

fn format_mention_string(mentions: &[&(dyn Mentionable + Sync)]) -> String {
    mentions
        .iter()
        .fold(String::new(), |acc, x| format!("{} {}", acc, x.mention()))
}

#[instrument(skip(http, mention))]
pub async fn repeat_mention_multiple(
    http: &impl AsRef<Http>,
    channel: ChannelId,
    mention: &[&(dyn Mentionable + Sync)],
    count: usize,
    delay: Duration,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    for _ in 0..count {
        channel.say(http, format_mention_string(mention)).await?;

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
