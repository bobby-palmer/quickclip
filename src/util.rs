pub fn get_folder_name() -> String {
    let fullpath = std::env::current_dir().unwrap();
    String::from(fullpath.file_name().unwrap().to_str().unwrap())
}
