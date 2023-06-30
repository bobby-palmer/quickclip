use clap::{Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeMap, env, fs, path::PathBuf, str::FromStr};
use toml;

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

#[derive(Deserialize, Serialize)]
struct Index(BTreeMap<String, PathBuf>);
impl Index {
    fn from_file(file: PathBuf) -> Self {
        let content = fs::read_to_string(file).unwrap();
        toml::from_str(&content).unwrap()
    }
    fn add(&mut self, alias: String) {
        self.0
            .insert(alias, env::current_dir().expect("error getting directory"));
    }
    fn remove(&mut self, alias: String) {
        self.0.remove(&alias);
    }
    fn list(&self) {
        self.0.iter().for_each(|(key, val)| {
            println!("{} in place of : {}", key, val.to_str().unwrap());
        })
    }
    fn to_file(&self, file: PathBuf) {
        let index = toml::to_string_pretty(self).unwrap();
        fs::write(file, index).unwrap();
    }
}

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
