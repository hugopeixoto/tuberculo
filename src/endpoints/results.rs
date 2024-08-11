use crate::database::Database;
use crate::DatabaseState;

#[derive(askama_axum::Template)]
#[template(path = "results.html")]
pub struct Template {
    videos: Vec<crate::database::Video>,
}

#[derive(serde::Deserialize)]
pub struct Results {
    term: String,
}

pub async fn handler(
    db: axum::extract::State<DatabaseState>,
    axum::Form(query): axum::Form<Results>,
) -> Template {
    let db = db.read().unwrap();
    Template {
        videos: db.search(&query.term),
    }
}
