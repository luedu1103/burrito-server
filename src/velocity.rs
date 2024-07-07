use std::time::Duration;
use rocket::{http::Status, Route};
use crate::Message;
use rocket::State;
use rocket::serde::json::{json, Value};

use crate::PositionState;

pub fn routes() -> Vec<Route> {
    routes![get_velocity]
}

fn calculate_velocity(positions: &[Message]) -> f64 {
    if positions.len() < 2 {
        return 0.0;
    }

    let mut total_distance = 0.0;
    let mut total_time = Duration::new(0, 0);

    for i in 1..positions.len() {
        let pos1 = &positions[i - 1];
        let pos2 = &positions[i];

        let lat1 = pos1.lt;
        let lon1 = pos1.lg;
        let lat2 = pos2.lt;
        let lon2 = pos2.lg;

        let distance = haversine(lat1, lon1, lat2, lon2);
        total_distance += distance;

        let time_diff = pos2.timestamp.unwrap().duration_since(pos1.timestamp.unwrap()).unwrap_or(Duration::new(0, 0));
        total_time += time_diff;
    }

    if total_time.as_secs() == 0 {
        return 0.0;
    }

    // Convert total time to hours and calculate speed in km/h
    let total_time_hours = total_time.as_secs_f64() / 3600.0;
    total_distance / total_time_hours
}

fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let rad = 6371.0; // Radius of the Earth in kilometers
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();

    let a = (d_lat / 2.0).sin().powi(2) +
            lat1.to_radians().cos() * lat2.to_radians().cos() *
            (d_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    rad * c // Distance in kilometers
}

#[get("/get-velocity")]
fn get_velocity(state: &State<PositionState>) -> Result<Value, Status> {
    let messages = state.messages.lock().unwrap();
    if messages.is_empty() {
        return Err(Status::InternalServerError);
    }

    let velocity = calculate_velocity(&messages);
    Ok(json!({
        "velocity": velocity,
    }))
}