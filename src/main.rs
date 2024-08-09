use axum::routing::{get, post};
use axum::Router;

pub mod database;
mod endpoints;
mod schema;

type DatabaseState = std::sync::Arc<std::sync::RwLock<database::Sqlite3>>;

#[tokio::main]
async fn main() {
    let shared_state = std::sync::Arc::new(std::sync::RwLock::new(database::Sqlite3::new(
        "db/test.sqlite3",
    )));

    // build our application with a route
    let app = Router::new()
        .route("/", get(endpoints::root::handler))
        .route("/results", get(endpoints::results::handler))
        .route("/enqueue", post(endpoints::enqueue::handler))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
