use std::{
    io::Error,
    process::{Command, Output},
};
pub fn get_folder_name() -> String {
    let fullpath = std::env::current_dir().unwrap();
    String::from(fullpath.file_name().unwrap().to_str().unwrap())
}
