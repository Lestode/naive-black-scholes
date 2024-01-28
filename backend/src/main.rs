mod data_fetcher;
mod european_options;
mod routes;
use axum::{routing::get, Router};
use european_options::Option;
use routes::router;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    // TODO: create the route file
    let app = router();
    let listener = match TcpListener::bind("0.0.0.0:8000").await {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("Failed to bind to address: {}", error);
            return;
        }
    };
    if let Err(error) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", error);
    }
}
