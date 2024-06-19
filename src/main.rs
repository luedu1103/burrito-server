use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Message {
    latitud: String,
    longitud: String,
    // velocidad: String,
}

#[derive(Default)]
struct PositionState {
    message: Mutex<Option<Message>>,
}

#[get("/get-position")]
fn get_position(state: &State<PositionState>) -> String {
    let message = state.message.lock().unwrap();
    if let Some(msg) = &*message {
        format!("https://www.google.com/maps?q={},{}", msg.latitud, msg.longitud)
    } else {
        "No position data available".to_string()
    }
}

#[post("/give-position", format = "json", data = "<message_json>")]
fn give_position(message_json: Json<Message>, state: &State<PositionState>) -> Status {
    let mut message = state.message.lock().unwrap();
    *message = Some(message_json.into_inner());
    Status::Ok
}

#[options("/give-position")]
fn handle_options_request() -> Status {
    Status::NoContent
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index, give_position, handle_options_request, get_position])
        .manage(PositionState::default());

    Ok(rocket.into())
}
