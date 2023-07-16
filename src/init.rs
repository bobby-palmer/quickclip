use std::fs::File;

use crate::defaults;

// used to create new config file and eventually will be able to create the goto function for most
// major shells
pub fn init() -> bool {
    if defaults::get_config().is_ok(){
        false
    }else {
        // no current config
        let file = dirs::config_dir().unwrap().join("mark/config.toml");
        let _ = File::create(file);
        true
    }
}
