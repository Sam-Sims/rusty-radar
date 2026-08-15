use std::{env, sync::Arc};

use anyhow::Result;
use askama::Template;
use axum::{Json, Router, extract::State, http::StatusCode, response::Html, routing::get};
use geo::{Bearing, Distance, Geodesic, Point};
use serde::Serialize;
use tokio::sync::RwLock;

mod antenna;
mod web;

// pub static MY_LAT: LazyLock<Result<f64>> = LazyLock::new(|| {
//     env::var("RUSTY_RADAR_LAT")
//         .context("RUSTY_RADAR_LAT env var must be set")?
//         .parse::<f64>()
//         .context("Cannot parse RUSTY_RADAR_LAT to float")
// });

// pub static MY_LON: LazyLock<Result<f64>> = LazyLock::new(|| {
//     env::var("RUSTY_RADAR_LON")
//         .context("RUSTY_RADAR_LON env var must be set")?
//         .parse::<f64>()
//         .context("Cannot parse RUSTY_RADAR_LON to float")
// });

pub const DIST: f32 = 16.20;

use crate::web::poll_web_aicraft;

#[derive(Clone)]
struct AppState {
    data_source: DataSource,
    aircraft: Arc<RwLock<Vec<Aircraft>>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PositionOffset {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecieverLocation {
    lat: f64,
    lon: f64,
}

impl PositionOffset {
    pub fn new(my_lat: f64, my_lon: f64, aircraft_lat: f64, aircraft_lon: f64) -> Self {
        let me = Point::new(my_lon, my_lat);
        let aircraft = Point::new(aircraft_lon, aircraft_lat);

        let distance = Geodesic.distance(me, aircraft);
        let bearing = Geodesic.bearing(me, aircraft).to_radians();

        PositionOffset {
            x: distance * bearing.sin(),
            y: distance * bearing.cos(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Aircraft {
    hex: String,
    callsign: String,
    aircraft_type: String,
    description: String,
    altitude: u32,
    flight_level: String,
    squawk: Option<String>,
    lat: f64,
    lon: f64,
    track: Option<f32>,
    label: String,
    timestamp: u64,
    position_offset: PositionOffset,
}

#[derive(Clone, Copy, Debug)]
pub enum DataSource {
    Antenna,
    Web,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

async fn index() -> Result<Html<String>, StatusCode> {
    let template = IndexTemplate { name: "SAM" };
    let html = template.render().map_err(|error| {
        tracing::error!("failure {:?}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(html))
}

async fn spawn_ingest_loop(state: AppState, reciver_location: RecieverLocation) {
    let client = reqwest::Client::new();

    loop {
        let result = match state.data_source {
            DataSource::Web => poll_web_aicraft(&client, &reciver_location).await,
            DataSource::Antenna => !unimplemented!(),
        };

        match result {
            Ok(aircraft) => {
                *state.aircraft.write().await = aircraft;
            }
            Err(error) => {
                tracing::error!("poll failed")
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}

async fn get_aircraft(State(state): State<AppState>) -> Json<Vec<Aircraft>> {
    let aircraft = state.aircraft.read().await.clone();
    Json(aircraft)
}

async fn status() {}

#[tokio::main]
async fn main() {
    let data_dir = env::var("RUSTY_RADAR_DATA");
    let my_lon = env::var("RUSTY_RADAR_LON").unwrap().parse::<f64>().unwrap();
    let my_lat = env::var("RUSTY_RADAR_LAT").unwrap().parse::<f64>().unwrap();
    let my_location = RecieverLocation {
        lat: my_lat,
        lon: my_lon,
    };

    let aircraft = Arc::new(RwLock::new(Vec::new()));

    let state = AppState {
        data_source: DataSource::Web,
        aircraft: aircraft,
    };
    tokio::spawn(spawn_ingest_loop(state.clone(), my_location));

    let api = Router::new()
        .route("/aircraft", get(get_aircraft))
        .route("/status", get(status))
        .with_state(state);

    let app = Router::new().route("/", get(index)).nest("/api", api);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Runnon server on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
