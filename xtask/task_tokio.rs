use anyhow::Context;
use std::{ffi::OsStr, process::Stdio};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, BufReader},
    process::Command,
};

pub async fn build(release: bool) -> anyhow::Result<()> {
    let args = if release {
        vec!["build", "--release"]
    } else {
        vec!["build"]
    };
    run(Command::new("cargo"), args)
        .await
        .context("Failed to run cargo build")?;
    Ok(())
}

pub async fn run<I, S>(mut cmd: Command, args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut child = cmd
        .args(args)
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    if let Some(stdout) = child.stdout.take() {
        println_lines(stdout).await?;
    } else if let Some(stderr) = child.stderr.take() {
        println_lines(stderr).await?;
    }
    Ok(())
}

async fn println_lines<T>(inner: T) -> anyhow::Result<()>
where
    T: AsyncRead + Unpin,
{
    let mut lines = BufReader::new(inner).lines();
    while let Some(line) = lines.next_line().await? {
        println!("{}", line);
    }
    Ok(())
}
