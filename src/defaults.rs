use std::path::PathBuf;

/// Returns the path to the systems config file
/// or an error if it cannot find the path
///
pub fn get_config() -> Result<PathBuf, &'static str> {
    if let Some(base) = dirs::config_dir() {
        let path = base.join("mark/config.toml");
        if path.is_file() {
            return Ok(path);
        }
    }
    if let Some(base) = dirs::home_dir() {
        let path = base.join(".config/mark/config.toml");
        if path.is_file() {
            return Ok(path);
        }
    }
    return Err("Cannot find config file!\nconsider running quickclip init");
}
