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

#[launch]
fn rocket() -> _ {
    println!("Parsed, {:?}", CONFIG.ytdlp);
    let result = ytdlp::app_call(ytdlp::Command::Search {
        id: "name".to_string(),
    });
    println!("{:?}", result);
    rocket::build().mount("/", routes![index])
}
