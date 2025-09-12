use anyhow::Context;
use std::{
    ffi::OsStr,
    io::{BufRead, BufReader, Read},
    process::{Command, Stdio},
};

pub fn build(release: bool) -> anyhow::Result<()> {
    let args = if release {
        vec!["build", "--release"]
    } else {
        vec!["build"]
    };
    run(Command::new("cargo"), args).context("Failed to run cargo build")?;
    Ok(())
}

pub fn run<I, S>(mut cmd: Command, args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut child = cmd
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    if let Some(stdout) = child.stdout.take() {
        println_lines(stdout)?;
    } else if let Some(stderr) = child.stderr.take() {
        println_lines(stderr)?;
    }
    Ok(())
}

fn println_lines<T>(inner: T) -> anyhow::Result<()>
where
    T: Read + Unpin,
{
    let lines = BufReader::new(inner).lines();
    for line in lines {
        println!("{}", line?);
    }
    Ok(())
}
