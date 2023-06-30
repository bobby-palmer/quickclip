use std::process::Command;
pub fn get_folder_name() -> String {
    let fullpath = std::env::current_dir().unwrap();
    String::from(fullpath.file_name().unwrap().to_str().unwrap())
}
pub fn change_directory(new_dir: &String) {
    Command::new("cd ".to_owned() + new_dir).output().unwrap();
}
