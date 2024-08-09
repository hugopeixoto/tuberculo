pub mod sqlite3;

pub use sqlite3::Sqlite3;

#[derive(diesel::Queryable, diesel::Selectable, diesel::Insertable)]
#[diesel(table_name = crate::schema::videos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Clone, Debug)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub duration: i32,
    pub aspect_ratio: f32,
    pub fulltitle: Option<String>,
    pub categories: Option<String>,
    pub full_metadata: String,
    pub fetched_at: chrono::NaiveDateTime,
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
    fn fail(&mut self, id: i32);
    fn store_metadata(&mut self, _metadata: &Video) -> Result<(), anyhow::Error>;
}
