use clap::{Parser, Subcommand};
#[derive(Subcommand)]
pub enum Commands {
    Init{ name: String },
    List,
    Mark { name: Option<String> },
    Remove { name: String },
    Goto { alias: Option<String> },
}

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}
