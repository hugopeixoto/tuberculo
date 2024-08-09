pub mod memory;
pub mod sqlite3;

pub use memory::Memory;
pub use sqlite3::Sqlite3;

#[derive(Clone, Debug)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: String,
}

pub trait Database {
    fn search(&self, term: &str) -> Vec<Video>;
    fn enqueue(&mut self, url: String);
    fn queue_size(&self) -> usize;
}
