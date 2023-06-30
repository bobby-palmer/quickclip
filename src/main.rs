use clap::Parser;
use mark::index::Index;
use mark::input::{Cli, Commands};
use std::{path::PathBuf, str::FromStr};

fn list() {}

fn mark(alias: Option<String>) {
    let filepath = PathBuf::from_str("/Users/bobbypalmer/Desktop/marks.toml");
    let out = PathBuf::from_str("/Users/bobbypalmer/Desktop/out.toml");
    let mut index = Index::from_file(filepath.unwrap());
    index.add(String::from("Test"));
    index.to_file(out.unwrap());
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Some(Commands::List) => list(),
        Some(Commands::Mark { name }) => mark(name.clone()),
        None => mark(None),
    }
}
