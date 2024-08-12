// SPDX-FileCopyrightText: 2024 Hugo Peixoto <hugo.peixoto@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

use crate::database::Database;
use crate::DatabaseState;

#[derive(serde::Deserialize)]
pub struct Enqueue {
    urls: String,
}

pub async fn post(
    db: axum::extract::State<DatabaseState>,
    axum::Form(enqueue): axum::Form<Enqueue>,
) -> axum::response::Redirect {
    let mut db = db.write().unwrap();

    for url in enqueue.urls.trim().lines() {
        db.enqueue(&url.trim());
    }

    axum::response::Redirect::to("/enqueue")
}

#[derive(askama_axum::Template)]
#[template(path = "enqueue.html")]
pub struct Template {
    stats: crate::database::Stats,
}

pub async fn get(db: axum::extract::State<DatabaseState>) -> Template {
    let db = db.read().unwrap();
    Template { stats: db.stats() }
}
