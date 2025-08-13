use super::error::{Error::PlayFailed, Result};
use super::series::Episode;
use std::process::Command;

pub fn play(episode: &Episode) -> Result<()> {
    let output = match Command::new("mpv").arg(episode.addr.to_string()).output() {
        Ok(o) => o,
        Err(_) => return Err(PlayFailed("Failed to execute mpv".to_string())),
    };
    if !output.status.success() {
        return Err(PlayFailed("Can't open mpv".to_string()));
    }
    Ok(())
}
