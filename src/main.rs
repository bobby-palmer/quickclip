use clap::Parser;
use quickclip::defaults;
use quickclip::index::Index;
use quickclip::input::{Cli, Commands};
use quickclip::util;
use quickclip::gui::launch_gui;
use quickclip::init;

fn list() {
    let filepath = defaults::get_config().unwrap();
    let index = Index::from_file(&filepath);
    index.list();
}

fn mark(alias: Option<String>) {
    let filepath = defaults::get_config().unwrap();
    let mut index = Index::from_file(&filepath);
    let clip_name = alias.unwrap_or_else(|| util::get_folder_name());
    if index.add(
        clip_name.clone(),
        std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned(),
    ) {
        println!("Directory successfully bookmarked");
    } else {
        println!("A bookmark with name {clip_name} already exists!");
        println!("Try giving this one a different name with `quickclip NAME`");
    }
    index.to_file(&filepath);
}

fn remove(alias: String) {
    let filepath = defaults::get_config().unwrap();
    let mut index = Index::from_file(&filepath);
    index.remove(alias);
    index.to_file(&filepath);
}

fn goto(alias: Option<String>) {
    let filepath = defaults::get_config().unwrap();
    let index = Index::from_file(&filepath);
    if let Some(name) = alias {
        let fullpath = index.get(name).expect("must be a valid mark name!");
        println!("{fullpath}")
    } else {
        launch_gui(index.out());
    }
}

fn init() {
    init::init();
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Some(Commands::Init) => init(),
        Some(Commands::List) => list(),
        Some(Commands::Mark { name }) => mark(name.clone()),
        Some(Commands::Remove { name }) => remove(name.clone()),
        Some(Commands::Goto { alias }) => goto(alias.clone()),
        None => mark(None),
    }
}
