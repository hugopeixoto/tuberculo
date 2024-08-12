// SPDX-FileCopyrightText: 2024 Hugo Peixoto <hugo.peixoto@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

use crate::database::Database;
use crate::DatabaseState;
use axum::debug_handler;

use axum::extract::{Path, State};
use axum::http::HeaderMap;

#[derive(askama_axum::Template)]
#[template(path = "watch.fragment.html")]
pub struct TemplateFragment {
    video: crate::database::Video,
    next_video: Option<crate::database::Video>,
    autoplay: bool,
}

#[derive(askama_axum::Template)]
#[template(path = "watch.html")]
pub struct Template {
    stats: crate::database::Stats,
    video: crate::database::Video,
    next_video: Option<crate::database::Video>,
    autoplay: bool,
}

#[debug_handler]
pub async fn handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    db: State<DatabaseState>,
) -> axum::response::Html<String> {
    let db = db.read().unwrap();

    let htmx = headers
        .get("HX-Request")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("false")
        == "true";

    let video = db.get(&id).unwrap();
    let next_video = db.next(&video).ok();
    let stats = db.stats();

    if htmx {
        axum::response::Html(
            askama_axum::Template::render(&TemplateFragment {
                video,
                next_video,
                autoplay: true,
            })
            .unwrap(),
        )
    } else {
        axum::response::Html(
            askama_axum::Template::render(&Template {
                stats,
                video,
                next_video,
                autoplay: false,
            })
            .unwrap(),
        )
    }
}
