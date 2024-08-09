use crate::database::Database;

pub fn download(db: &crate::DatabaseState) {
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
            )
            .unwrap()
            .download();

            match x {
                Ok(y) => {
                    println!("downloaded {}", y.output());
                    let mut connection = db.write().unwrap();

                    connection.store_metadata(y.output());
                    connection.done(id);
                }
                Err(_) => {
                    db.write().unwrap().fail(id);
                }
            }
        }
    }
}
