use clap::Parser;
use quickclip::defaults;
use quickclip::index::Index;
use quickclip::input::{Cli, Commands};
use quickclip::util;

fn list() {
    let filepath = defaults::get_config().unwrap();
    let index = Index::from_file(&filepath);
    index.list();
}

fn mark(alias: Option<String>) {
    let filepath = defaults::get_config().unwrap();
    let mut index = Index::from_file(&filepath);
    index.add(
        alias.unwrap_or_else(|| util::get_folder_name()),
        std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned(),
    );
    index.to_file(&filepath);
}

fn remove(alias: String) {
    let filepath = defaults::get_config().unwrap();
    let mut index = Index::from_file(&filepath);
    index.remove(alias);
    index.to_file(&filepath);
}

fn goto(alias: String) {
    let filepath = defaults::get_config().unwrap();
    let index = Index::from_file(&filepath);
    let fullpath = index.get(alias);
    util::change_directory(fullpath.unwrap());
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Some(Commands::List) => list(),
        Some(Commands::Mark { name }) => mark(name.clone()),
        Some(Commands::Remove { name }) => remove(name.clone()),
        Some(Commands::goto { alias }) => goto(alias.clone()),
        None => mark(None),
    }
}
