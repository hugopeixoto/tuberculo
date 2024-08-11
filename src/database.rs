// SPDX-FileCopyrightText: 2024 Hugo Peixoto <hugo.peixoto@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

pub mod sqlite3;

pub use sqlite3::Sqlite3;

#[derive(
    serde::Deserialize, Debug, Clone, diesel::Queryable, diesel::Selectable, diesel::Insertable,
)]
#[diesel(table_name = crate::schema::videos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub duration: i32,
    pub aspect_ratio: f32,
    pub fulltitle: Option<String>,
    pub categories: Option<String>,
    pub thumbnail_extension: String,
    #[serde(skip_deserializing)]
    pub full_metadata: String,
    #[serde(skip_deserializing)]
    pub fetched_at: chrono::NaiveDateTime,
}

impl Video {
    pub fn from_json(metadata: &str) -> anyhow::Result<Self> {
        let json: VideoJSON = serde_json::from_str(metadata)?;

        println!("metadata: {:?}", json);
        Ok(Self {
            id: json.id.clone(),
            title: json.title,
            description: json.description,
            duration: json.duration,
            aspect_ratio: json.aspect_ratio,
            fulltitle: json.fulltitle,
            categories: Some(json.categories.join(", ")),
            thumbnail_extension: json
                .thumbnails
                .last()
                .ok_or(anyhow::format_err!(
                    "no thumbnail available for {}",
                    json.id,
                ))?
                .url
                .split(".")
                .last()
                .unwrap_or("jpg")
                .into(),
            full_metadata: metadata.into(),
            fetched_at: chrono::Utc::now().naive_utc(),
        })
    }
}

#[derive(Clone, Debug)]
pub enum Job {
    Download(i32, String),
}

pub trait Database {
    fn search(&self, term: &str) -> Vec<Video>;
    fn enqueue(&mut self, url: String);
    fn queue_size(&self) -> usize;
    fn pop_queue(&mut self) -> Result<Job, anyhow::Error>;
    fn done(&mut self, id: i32);
    fn fail(&mut self, id: i32, err: &anyhow::Error);
    fn store_metadata(&mut self, _metadata: &Video) -> Result<(), anyhow::Error>;
    fn get(&self, id: &String) -> Result<Video, anyhow::Error>;
    fn next(&self, video: &Video) -> Result<Video, anyhow::Error>;
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct VideoJSON {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub duration: i32,
    pub aspect_ratio: f32,
    pub fulltitle: Option<String>,
    pub categories: Vec<String>,
    pub thumbnails: Vec<ThumbnailJSON>,
    #[serde(skip_deserializing)]
    pub full_metadata: String,
    #[serde(skip_deserializing)]
    pub fetched_at: chrono::NaiveDateTime,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ThumbnailJSON {
    pub url: String,
}
