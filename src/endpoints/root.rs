// SPDX-FileCopyrightText: 2024 Hugo Peixoto <hugo.peixoto@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

use crate::database::Database;
use crate::DatabaseState;

#[derive(askama_axum::Template)]
#[template(path = "root.html")]
pub struct Template {
    videos: Vec<crate::database::Video>,
    queue_size: usize,
}

pub async fn handler(db: axum::extract::State<DatabaseState>) -> Template {
    let db = db.write().unwrap();
    Template {
        videos: db.search(""),
        queue_size: db.queue_size(),
    }
}
