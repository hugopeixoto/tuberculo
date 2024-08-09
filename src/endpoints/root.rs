use crate::database::Database;
use crate::DatabaseState;

#[derive(askama_axum::Template)]
#[template(path = "root.html")]
pub struct Template {
    videos: Vec<crate::database::Video>,
    queue_size: usize,
}

pub async fn handler(db: axum::extract::State<DatabaseState>) -> Template {
    let db = db.read().unwrap();
    Template {
        videos: db.search("hello"),
        queue_size: db.queue_size(),
    }
}
