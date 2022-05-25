#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

mod settings;
mod ytdlp;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/id/<videoid>")]
async fn get_by_id(videoid: String) {
    let result = ytdlp::req_by_link(videoid);
    println!("{:?}", result);
}

#[get("/search/<searchstring>")]
async fn run_search(searchstring: String) {
    let result = ytdlp::search(searchstring);
    println!("{:?}", result);
}

#[launch]
fn rocket() -> _ {
    println!("Parsed, {:?}", CONFIG.ytdlp);
    rocket::build().mount("/", routes![index, get_by_id, run_search])
}
