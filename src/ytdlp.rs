// yt-dlp interaction
use crate::CONFIG;
use serde::Deserialize;
use std::process::Command as PCommand;

#[cfg_attr(test, derive(Clone, PartialEq, Debug))]
pub enum Command {
    Link { id: String },
    Search { string: String },
}

#[derive(Deserialize, Debug)]
pub struct VideoFormat {
    format_id: String,
    format_note: String,
    ext: String,
    acodec: String,
    vcodec: String,
    width: Option<i64>,
    height: Option<i64>,
}

#[derive(Deserialize)]
pub struct Video {
    id: String,
    title: String,
    formats: Vec<VideoFormat>,
    format: String,
}

#[cfg_attr(test, derive(Debug, Clone))]
pub enum YtDlpError {
    OutputError,
    JsonError(std::sync::Arc<serde_json::Error>),
}

impl From<serde_json::Error> for YtDlpError {
    fn from(err: serde_json::Error) -> YtDlpError {
        YtDlpError::JsonError(std::sync::Arc::new(err))
    }
}

#[cfg_attr(test, mry::mry)]
fn app_call(cmd: Command) -> Result<String, YtDlpError> {
    let mut args = vec!["--dump-json".to_string(), "--no-progress".to_string()];

    match cmd {
        Command::Search { string } => args.push(format!("ytsearch:{}", string).to_string()),
        Command::Link { id } => args.push(format!("{}", id).to_string()),
    };

    let result = PCommand::new(&CONFIG.ytdlp.path)
        .args(args)
        .output()
        .expect("failed to execute process");
    if let Ok(out) = String::from_utf8(result.stdout) {
        Ok(out)
    } else {
        Err(YtDlpError::OutputError)
    }
}

pub fn req_by_link(videoid: String) -> Result<String, YtDlpError> {
    let json = app_call(Command::Link { id: videoid })?;
    let json = json.trim_start_matches(|c| c != '{');
    let v: Video = serde_json::from_str(&json)?;
    println!("{:?}", v.formats);
    println!("{:?}", &CONFIG.env);
    Ok(v.title)
}

pub fn search(searchstring: String) {
    app_call(Command::Search {
        string: searchstring,
    });
}

#[cfg(test)]
mod test {
    use super::*;
    use mry::Any;
    use std::path::PathBuf;

    #[test]
    #[mry::lock(app_call)]
    fn test_req_by_link() -> Result<(), YtDlpError> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/id_req.json");
        let testdata = std::fs::read_to_string(d).unwrap();

        mock_app_call(Any).returns(Ok(testdata));

        assert_eq!(req_by_link(String::new())?, String::from("result"));
        Ok(())
    }
}
