use axum::routing::{get, post};
use axum::Router;

pub mod database;
mod downloader;
mod endpoints;
mod schema;

type DatabaseState = std::sync::Arc<std::sync::RwLock<database::Sqlite3>>;

#[tokio::main]
async fn main() {
    // options
    let db_path = "db/test.sqlite3";
    let bind_address = "0.0.0.0:3000";

    let shared_state = std::sync::Arc::new(std::sync::RwLock::new(database::Sqlite3::new(db_path)));

    let db = shared_state.clone();
    std::thread::spawn(move || loop {
        downloader::download(&db);
        std::thread::sleep(std::time::Duration::from_millis(10_000));
    });

    // build our application with a route
    let app = Router::new()
        .route("/", get(endpoints::root::handler))
        .route("/results", get(endpoints::results::handler))
        .route("/enqueue", post(endpoints::enqueue::handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
