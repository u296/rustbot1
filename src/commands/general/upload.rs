use super::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[command]
async fn upload(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    info!("upload");
    if msg.attachments.is_empty() {
        msg.channel_id.say(ctx, "no attached file").await?;
        return Ok(());
    }

    let attachment = &msg.attachments[0];

    let savename = args.message();

    let content = {
        use std::io;
        match tokio::fs::File::open("content/manifest.json").await {
            Ok(mut f) => {
                let mut bytes = Vec::new();
                f.read_to_end(&mut bytes).await?;

                String::from_utf8(bytes).unwrap()
            }
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                tokio::fs::create_dir_all("content").await?;
                String::from("{}")
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    };

    let mut table: utils::ContentManifest = serde_json::from_str(&content)?;

    table.uploads.insert(savename.into(), format!("{}.opus", attachment.filename));

    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("content/manifest.json")
        .await?;

    file.write_all(&serde_json::to_vec(&table)?).await?;

    let bytes = attachment.download().await?;

    let tmp_dir = "/tmp/rustbot";
    let tmp_file = format!("/tmp/rustbot/{}", attachment.filename);
    let converted_file = format!("content/{}.opus", attachment.filename);

    tokio::fs::create_dir_all(tmp_dir).await?;
    tokio::fs::write(tmp_file.clone(), &bytes).await?;

    async_process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(tmp_file)
        .arg(converted_file)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()?
        .status()
        .await?;

    msg.channel_id
        .say(
            ctx,
            format!(
                "successfully uploaded {} as {}",
                attachment.filename, savename
            ),
        )
        .await?;
    info!("successfully uploaded {}", attachment.filename);

    Ok(())
}

#[command]
async fn list(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    match tokio::fs::File::open("content/manifest.json").await {
        Ok(mut f) => {
            let mut bytes = Vec::new();
            f.read_to_end(&mut bytes).await?;

            let list = serde_json::from_slice::<utils::ContentManifest>(&bytes)?
                .uploads
                .iter()
                .map(|(key, _)| key)
                .cloned()
                .collect::<Vec<String>>()
                .join("\n");

            msg.channel_id.say(ctx, list).await?;

            Ok(())
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            msg.channel_id.say(ctx, "nothing saved").await?;
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
