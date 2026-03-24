mod lcd;
mod menu;

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Vehicle {
    id: String,
    trip_id: Option<String>,
    position: Position,
    stop_id: Option<String>,
    current_status: Option<String>, // e.g., "IN_TRANSIT_TO", "STOPPED_AT"
}

#[derive(Deserialize, Debug)]
struct Position {
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    items: Vec<Vehicle>,
}

// wss://api.metro.net/ws/lacmts-rail/vehicle-positions/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    menu::update();

    Ok(())
}
