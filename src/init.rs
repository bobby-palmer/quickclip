use std::fs::File;

use crate::defaults;

const FISH: &'static str = include_str!("./resources/scripts/init.fish");

// used to create new config file and eventually will be able to create the goto function for most
// major shells
pub fn init() {
    if defaults::get_config().is_ok(){
        ()
    }else {
        // no current config
        let file = dirs::config_dir().unwrap().join("mark/config.toml");
        let _ = File::create(file);
    }
    println!("{FISH}");
}
