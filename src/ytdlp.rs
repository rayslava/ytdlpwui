// yt-dlp interaction
use crate::CONFIG;
use rocket::serde::Serialize;
use serde::Deserialize;
use std::process::Command as PCommand;

#[cfg_attr(test, derive(Clone, PartialEq, Debug))]
pub enum Command {
    Link { id: String },
    Search { string: String, number: u16 },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoFormat {
    format_id: String,
    format_note: String,
    ext: String,
    acodec: String,
    vcodec: String,
    width: Option<i64>,
    height: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
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
        Command::Search { string, number } => args.push(format!("ytsearch{}:{}", number, string)),
        Command::Link { id } => args.push(id),
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

pub fn req_by_link(videoid: String) -> Result<Video, YtDlpError> {
    let json = app_call(Command::Link { id: videoid })?;
    let json = json.trim_start_matches(|c| c != '{');
    let v: Video = serde_json::from_str(json)?;
    Ok(v)
}

pub fn search(searchstring: String) -> Result<Vec<Video>, YtDlpError> {
    let json = app_call(Command::Search {
        string: searchstring,
        number: CONFIG.ytdlp.search_num,
    })?;
    let result: Vec<Video> = json
        .split('\n')
        .filter_map(|line| serde_json::from_str(line).ok()?)
        .collect();
    println!("Found {} results", result.len());
    Ok(result)
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

        let test_v = req_by_link(String::new())?;

        assert_eq!(test_v.title, String::from("КАК ЧИТАТЬ ДОРОГУ?"));
        Ok(())
    }

    #[test]
    #[mry::lock(app_call)]
    fn test_search() -> Result<(), YtDlpError> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/search_10.json");
        let testdata = std::fs::read_to_string(d).unwrap();
        mock_app_call(Any).returns(Ok(testdata));

        let test_v = search(String::new())?;

        assert_eq!(test_v.len(), 10);
        assert_eq!(
            test_v[0].title,
            String::from("Rick Astley - Never Gonna Give You Up (Official Music Video)")
        );
        assert_eq!(test_v[1].title, String::from("Rick - Наряд ангела"));
        Ok(())
    }
}
