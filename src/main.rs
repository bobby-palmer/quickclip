use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    List,
    Mark { name: Option<String> },
}

#[derive(Parser)]
#[clap(about, version, author)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}
fn init() {}

fn list() {}

fn mark(alias: Option<String>) {}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Some(Commands::List) => list(),
        Some(Commands::Mark { name }) => mark(name.clone()),
        None => mark(None),
    }
}
