// SPDX-FileCopyrightText: 2024 Hugo Peixoto <hugo.peixoto@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

use crate::database::Database;
use crate::DatabaseState;

#[derive(serde::Deserialize)]
pub struct Enqueue {
    url: String,
}

pub async fn handler(
    db: axum::extract::State<DatabaseState>,
    axum::Form(enqueue): axum::Form<Enqueue>,
) -> axum::response::Redirect {
    let mut db = db.write().unwrap();

    db.enqueue(enqueue.url);
    axum::response::Redirect::to("/")
}
