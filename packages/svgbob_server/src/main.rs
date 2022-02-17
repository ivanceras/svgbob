use axum::body::Bytes;
use axum::http::StatusCode;
use axum::{routing::get, Router};
use std::net::SocketAddr;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_PORT: u16 = 3000;

async fn hello() -> String {
    format!("Hello from svgbob_server {}", VERSION)
}

async fn text_to_svgbob(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(input) = String::from_utf8(body.to_vec()) {
        let svg = svgbob::to_svg(&input);
        Ok(svg)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .ok()
        .map(|port| port.parse::<u16>().ok())
        .flatten()
        .unwrap_or(DEFAULT_PORT);

    let app = Router::new().route("/", get(hello).post(text_to_svgbob));

    let socket: SocketAddr = ([0, 0, 0, 0], port).into();
    println!("serving at: {}", socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .expect("Error starting server");
}
