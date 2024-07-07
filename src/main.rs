use rocket::fs::{FileServer, relative};
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;
use rocket::{Request, Response};
use rocket::http::Header;
use rocket::fairing::{Fairing, Info, Kind};

#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

mod velocity;
mod position;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/mainpage")]
fn longlaoshi_main_page() -> Template {
    // Status::NoContent
    let mut context = HashMap::new();
    context.insert("hi","These are the static files c:");
    Template::render("index", &context)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    lt: f64,
    lg: f64,
    tmp: f32,
    hum: f32,
    sts: i32,
    #[serde(skip_deserializing)]
    timestamp: Option<SystemTime>,
}

#[derive(Default)]
pub struct PositionState {
    messages: Mutex<Vec<Message>>,
}

#[get("/loaderio-d0a8891a0d5f032a78809dc8605c4530")]
fn testing() -> String{
    let string = "loaderio-d0a8891a0d5f032a78809dc8605c4530".to_string();
    string
}

// CORS thing
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index, testing, longlaoshi_main_page])
        .mount("/", position::routes())
        .mount("/", velocity::routes())
        .mount("/static", FileServer::from(relative!("static")))
        .attach(CORS)
        .attach(Template::fairing())
        .manage(PositionState::default());

    Ok(rocket.into())
}

// Run without shuttle:
#[cfg(not(feature = "shuttle"))]
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, testing, longlaoshi_main_page])
    .mount("/", position::routes())
    .mount("/", velocity::routes())
    .mount("/static", FileServer::from(relative!("static")))
    .attach(CORS)
    .attach(Template::fairing())
    .manage(PositionState::default())
}
