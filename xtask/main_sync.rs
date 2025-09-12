use crate::task::build;
use clap::{Parser, Subcommand};

mod task;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    #[command(about = "Build this project")]
    Build {
        #[arg(short)]
        release: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    use crate::Action::*;
    match cli.action {
        Build { release } => build(release),
    }?;
    Ok(())
}
