use clap::Parser;
use mark::defaults;
use mark::index::Index;
use mark::input::{Cli, Commands};
use std::{path::PathBuf, str::FromStr};

fn list() {
    let filepath = defaults::get_config().unwrap();
    let index = Index::from_file(&filepath);
    index.list();
}

fn mark(alias: Option<String>) {
    let filepath = defaults::get_config().unwrap();
    let mut index = Index::from_file(&filepath);
    index.add(String::from("Test"), String::from("/Directoiry/to/test"));
    index.to_file(&filepath);
}

fn remove(alias: String) {
    let filepath = defaults::get_config().unwrap();
    let mut index = Index::from_file(&filepath);
    index.remove(alias);
    index.to_file(&filepath);
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Some(Commands::List) => list(),
        Some(Commands::Mark { name }) => mark(name.clone()),
        Some(Commands::Remove { name }) => remove(name.clone()),
        None => mark(None),
    }
}
