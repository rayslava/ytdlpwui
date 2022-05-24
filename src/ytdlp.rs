// yt-dlp interaction
use crate::CONFIG;

use std::process::Command as PCommand;

pub enum Command {
    Search { id: String },
}

pub fn app_call(cmd: Command) -> String {
    let result = PCommand::new(&CONFIG.ytdlp.path)
        .output()
        .expect("failed to execute process");
    if let Ok(out) = String::from_utf8(result.stdout) {
        out
    } else {
        "Something unbelievable".to_string()
    }
}
