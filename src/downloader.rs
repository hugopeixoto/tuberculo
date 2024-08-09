use crate::database::Database;

pub fn download(db: &crate::DatabaseState) -> Result<(), anyhow::Error> {
    let item = { db.write().unwrap().pop_queue() };

    match item {
        Err(err) => {
            println!("error fetching item from queue: {:?}", err);
        }
        Ok(crate::database::Job::Download(id, url)) => {
            println!("downloading.. {}", url);
            let x = ytd_rs::YoutubeDL::new(
                &std::path::PathBuf::from("videos/"),
                vec![
                    ytd_rs::Arg::new("--write-info-json"),
                    ytd_rs::Arg::new("--write-thumbnail"),
                    ytd_rs::Arg::new("-o%(id)s"),
                    ytd_rs::Arg::new("--dump-json"),
                    ytd_rs::Arg::new("--no-simulate"),
                ],
                &url,
            )?
            .download()
            .map_err(|e| anyhow::Error::new(e))
            .and_then(|download| {
                let metadata: Metadata = serde_json::from_str(&download.output())?;
                println!("downloaded {:?}", metadata);
                let mut connection = db
                    .write()
                    .map_err(|e| anyhow::format_err!("error fetching lock: {}", e))?;
                connection.store_metadata(&metadata.into(&download.output()))?;
                Ok(())
            });

            match x {
                Ok(()) => {
                    db.write().unwrap().done(id);
                }
                Err(e) => {
                    println!("Failed to download video: {:?}", e);
                    db.write().unwrap().fail(id);
                }
            }
        }
    }

    Ok(())
}

// this is silly, I could probably live with a single struct instead of three different layers for the same thing.
#[derive(serde::Deserialize, Debug, Clone)]
struct Metadata {
    pub id: String,
    pub description: String,
    pub title: String,
    pub fulltitle: String,
    pub categories: Vec<String>,
    pub duration: i32,
    pub aspect_ratio: f32,
}

// full_metadata TEXT NOT NULL,
// fetched_at TIMESTAMP NOT NULL

impl Metadata {
    fn into(self, full_metadata: &str) -> crate::database::Video {
        crate::database::Video {
            id: self.id,
            title: self.title,
            description: Some(self.description),
            fulltitle: Some(self.fulltitle),
            categories: Some(self.categories.join(", ")),
            duration: self.duration,
            aspect_ratio: self.aspect_ratio,
            full_metadata: full_metadata.into(),
            fetched_at: chrono::Utc::now().naive_utc(),
        }
    }
}
