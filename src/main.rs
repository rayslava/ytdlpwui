#[macro_use]
extern crate rocket;
use config::{Config, ConfigError};
use std::collections::HashMap;

#[derive(Debug)]
enum ConfError {
    Config(ConfigError),
    ParsingFail,
}

impl From<ConfigError> for ConfError {
    fn from(err: ConfigError) -> ConfError {
        ConfError::Config(err)
    }
}

fn parse_config() -> Result<HashMap<String, String>, ConfError> {
    let builder = Config::builder()
        .set_default("ytdlp", "/usr/bin/yt-dlp")?
        .add_source(config::File::new("ytdlpwui.conf", config::FileFormat::Toml))
        .add_source(config::Environment::with_prefix("YTDLPWUI"));

    if let Ok(settings) = builder.build() {
        let result = settings.try_deserialize::<HashMap<String, String>>();
        if let Ok(hashmap) = result {
            return Ok(hashmap);
        }
    }
    Err(ConfError::ParsingFail)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    if let Ok(result) = parse_config() {
        println!("Parsed, {:?}", result)
    };

    rocket::build().mount("/", routes![index])
}
