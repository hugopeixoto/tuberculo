use crate::database::Database;

pub fn download(db: &crate::DatabaseState, video_path: &str) -> Result<(), anyhow::Error> {
    let crate::database::Job::Download(id, url) = { db.write().unwrap().pop_queue() }?;

    let x = ytd_rs::YoutubeDL::new(
        &std::path::PathBuf::from(video_path),
        vec![
            ytd_rs::Arg::new("--write-info-json"),
            ytd_rs::Arg::new("--write-thumbnail"),
            ytd_rs::Arg::new("-o%(id)s"),
            ytd_rs::Arg::new("--print=%(id)s"),
            ytd_rs::Arg::new("--no-simulate"),
            ytd_rs::Arg::new("--merge-output-format=mp4"),
            ytd_rs::Arg::new("--prefer-free-formats"),
        ],
        &url,
    )?
    .download()
    .map_err(|e| anyhow::Error::new(e))
    .and_then(|download| {
        println!("reading {:?}", download.output());
        let info = std::fs::read_to_string(format!(
            "{}/{}.info.json",
            video_path,
            download.output().trim()
        ))?;
        let metadata = crate::database::Video::from_json(&info)?;

        let mut connection = db.write().unwrap();
        connection.store_metadata(&metadata)?;
        Ok(())
    });

    match &x {
        Ok(()) => {
            db.write().unwrap().done(id);
        }
        Err(e) => {
            println!("Failed to download video: {:?}", e);
            db.write().unwrap().fail(id);
        }
    }

    x
}
