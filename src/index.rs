use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Index(BTreeMap<String, PathBuf>);
impl Index {
    pub fn from_file(file: &PathBuf) -> Self {
        let content = fs::read_to_string(file).unwrap();
        toml::from_str(&content).unwrap()
    }
    pub fn add(&mut self, alias: String) {
        self.0
            .insert(alias, env::current_dir().expect("error getting directory"));
    }
    pub fn remove(&mut self, alias: String) {
        self.0.remove(&alias);
    }
    pub fn list(&self) {
        self.0.iter().for_each(|(key, val)| {
            println!("{} in place of : {}", key, val.to_str().unwrap());
        })
    }
    pub fn to_file(&self, file: &PathBuf) {
        let index = toml::to_string_pretty(self).unwrap();
        fs::write(file, index).unwrap();
    }
}
