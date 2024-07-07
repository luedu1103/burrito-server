use rocket::fs::{FileServer, relative};
use rocket::serde::json::{json, Value};
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;
use rocket::{Request, Response};
use rocket::http::Header;
use rocket::fairing::{Fairing, Info, Kind};

#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

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
struct Message {
    latitud: String,
    longitud: String,
    // velocidad: String,
}

#[derive(Default)]
struct PositionState {
    messages: Mutex<Vec<Message>>,
}

#[get("/get-position/<count>")]
fn get_position(count: usize, state: &State<PositionState>) -> Result<Value, Status> {
    let messages = state.messages.lock().unwrap();
    if messages.is_empty() {
        return Err(Status::InternalServerError);
    }
    
    let start = if messages.len() > count {
        messages.len() - count
    } else {
        0
    };

    let recent_messages: Vec<Message> = messages[start..].to_vec();
    Ok(json!({
        "positions": recent_messages,
    }))
}

#[post("/give-position", format = "json", data = "<message_json>")]
fn give_position(message_json: Json<Message>, state: &State<PositionState>) -> Status {
    let mut messages = state.messages.lock().unwrap();
    messages.push(message_json.into_inner());
    if messages.len() > 100 {
        messages.remove(0); // Keep only the latest 100 positions
    }
    Status::Ok
}

#[options("/give-position")]
fn handle_options_request() -> Status {
    Status::NoContent
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

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index, give_position, handle_options_request, get_position, testing, longlaoshi_main_page])
        .attach(CORS)
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .manage(PositionState::default());

    Ok(rocket.into())
}
