use std::process::Command;
use super::config_lua::series::Episode;

pub fn play(episode: &Episode) -> Result<(), String> {
    let output = match Command::new("mpv")
        .arg(episode.addr.to_string())
        .output() {
            Ok(o) => o,
            Err(_) => return Err("Failed to execute mpv".to_string()),
        };
    if !output.status.success() {
        return Err("Can't open mpv".to_string());
    }
    Ok(())
}
