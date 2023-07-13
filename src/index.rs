use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Index(BTreeMap<String, String>);
impl Index {
    pub fn from_file(file: &PathBuf) -> Self {
        let content = fs::read_to_string(file).unwrap();
        toml::from_str(&content).unwrap()
    }
    pub fn add(&mut self, alias: String, path: String) -> bool {
        if self.0.contains_key(&alias) {
            return false
        }
        self.0.insert(alias, path);
        true
    }
    pub fn remove(&mut self, alias: String) {
        self.0.remove(&alias);
    }
    pub fn list(&self) {
        self.0.iter().for_each(|(key, val)| {
            println!("{} in place of : {}", key, val);
        })
    }
    pub fn to_file(&self, file: &PathBuf) {
        let index = toml::to_string_pretty(self).unwrap();
        fs::write(file, index).unwrap();
    }
    pub fn get(&self, alias: String) -> Option<&String> {
        self.0.get(&alias)
    }
    pub fn out(self) -> BTreeMap<String, String> {
        self.0
    }
}
