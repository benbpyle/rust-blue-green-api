use axum::{response::Html, routing::get, Router, Json};
use axum::http::StatusCode;
use serde::{Serialize};
use tracing::info;


static COLOR: &str = "Green";

#[derive(Serialize)]
struct Payload {
    key: String,
}

async fn route_one() -> Json<Payload> {
    info!("Route One From {}", COLOR);
    Json(Payload { key: format!("route_one from {}", COLOR).to_string() } )
}

async fn route_two() -> Json<Payload> {
    info!("Route Two From {}", COLOR);
    Json(Payload { key: format!("route_two from {}", COLOR).to_string() } )
}

async fn route_three() -> Json<Payload> {
    info!("Route Three From {}", COLOR);
    Json(Payload { key: format!("route_three from {}", COLOR).to_string() } )
}

async fn route_error() -> Result<String, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

async fn health() -> Json<&'static str> {
    Json("Ping")
}


async fn home() -> Html<String> {
    Html(String::from("<html> <head> <title>Amazon ECS Sample App</title> <style>body {margin-top: 40px; background-color: #00BFFF;} </style> </head><body> <div style=color:white;text-align:center> <h1>Amazon ECS Sample App</h1> <h2>Congratulations!</h2> <p>Just some sample HTML</p> </div></body></html>"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
    // build our application with a single route
    let app = Router::new()
        .route("/", get(home), )
        .route("/one", get(route_one))
        .route("/two", get(route_two))
        .route("/three", get(route_three))
        .route("/error", get(route_error))
        .route("/health", get(health));

    info!("listening on 3000");
    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}