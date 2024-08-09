use askama_axum::Template;
use axum::routing::{get, post};
use axum::Router;

use axum::debug_handler;

use crate::database::Database;
use crate::database::Video;
mod database;
mod schema;

type DatabaseState = std::sync::Arc<std::sync::RwLock<database::Sqlite3>>;

#[tokio::main]
async fn main() {
    let shared_state = std::sync::Arc::new(std::sync::RwLock::new(database::Sqlite3::new(
        "db/test.sqlite3",
    )));

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/results", get(results))
        .route("/enqueue", post(enqueue))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {
    videos: Vec<Video>,
    queue_size: usize,
}

async fn root(db: axum::extract::State<DatabaseState>) -> RootTemplate {
    let db = db.read().unwrap();
    RootTemplate {
        videos: db.search("hello"),
        queue_size: db.queue_size(),
    }
}

#[derive(Template)]
#[template(path = "results.html")]
struct ResultsTemplate {
    videos: Vec<Video>,
}

async fn results(db: axum::extract::State<DatabaseState>) -> ResultsTemplate {
    let db = db.read().unwrap();
    ResultsTemplate {
        videos: db.search("hello"),
    }
}

#[derive(serde::Deserialize)]
struct Enqueue {
    url: String,
}

#[debug_handler]
async fn enqueue(
    db: axum::extract::State<DatabaseState>,
    axum::Form(enqueue): axum::Form<Enqueue>,
) -> axum::response::Redirect {
    let mut db = db.write().unwrap();

    db.enqueue(enqueue.url);
    axum::response::Redirect::to("/")
}
