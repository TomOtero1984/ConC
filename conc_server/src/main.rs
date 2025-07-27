use axum::{routing::get_service, Router};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let assets = PathBuf::from("../conc_sh/public");

    let app = Router::new()
        .nest_service("/", get_service(ServeDir::new(assets)));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind");

    println!("✅ Serving on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}