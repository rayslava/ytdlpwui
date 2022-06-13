#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

mod settings;
mod ytdlp;
use rocket::serde::json::Json;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GetVideoRequest<'r> {
    url: &'r str,
}

#[post("/get", format = "application/json", data = "<video>")]
async fn get_by_id<'l>(video: Json<GetVideoRequest<'l>>) -> Option<Json<ytdlp::Video>> {
    match ytdlp::req_by_link(video.url) {
        Ok(v) => Some(Json(v)),
        _ => None,
    }
}

#[post("/search", format = "application/json", data = "<searchstring>")]
async fn run_search<'l>(searchstring: &'l str) -> Option<Json<Vec<ytdlp::Video>>> {
    match ytdlp::search(searchstring) {
        Ok(v) => Some(Json(v)),
        _ => None,
    }
}

#[launch]
fn rocket() -> _ {
    println!("Parsed, {:?}", CONFIG.ytdlp);
    rocket::build().mount("/", routes![index, get_by_id, run_search])
}
