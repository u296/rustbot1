use super::prelude::*;

pub use futures::prelude::*;

#[command]
async fn exec(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if !ctx
        .data
        .read()
        .await
        .get::<crate::config::Config>()
        .expect("no config in typemap")
        .enable_exec
    {
        utils::send_buffered_text(
            ctx,
            msg.channel_id,
            stream::once(future::ready("command is disabled")),
        )
        .await?;
        return Ok(());
    }
    let cmdline = args.message();
    let mut cmdline = cmdline.split_whitespace();

    let mut command = async_process::Command::new(cmdline.next().unwrap());
    let mut child = command
        .args(cmdline)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout_lines = futures::io::BufReader::new(child.stdout.take().unwrap()).lines();
    let stderr_lines = futures::io::BufReader::new(child.stderr.take().unwrap()).lines();

    let output_lines =
        stream::select(stdout_lines, stderr_lines).filter_map(|r| future::ready(r.ok()));

    utils::send_buffered_text(ctx, msg.channel_id, output_lines).await
}
