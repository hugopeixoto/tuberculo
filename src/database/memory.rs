use rand::seq::SliceRandom;

pub struct Memory {
    pub videos: Vec<crate::database::Video>,
    pub queue: Vec<String>,
}

impl Memory {
    pub fn new() -> Self {
        let mut videos = vec![
            crate::database::Video {
                id: "21312".into(),
                title: "Kendrick Lamar - Not Like Us".into(),
                description: "Banger".into(),
            },
            crate::database::Video {
                id: "iuiwueraoiurywoieuy".into(),
                title: "Kendrick Lamar - DNA".into(),
                description: "Coolio".into(),
            },
        ];

        let mut rand = rand::thread_rng();
        videos.shuffle(&mut rand);

        return Self {
            videos: videos,
            queue: vec![],
        };
    }
}

impl crate::database::Database for Memory {
    fn search(&self, _term: &str) -> Vec<crate::database::Video> {
        self.videos.clone()
    }

    fn queue_size(&self) -> usize {
        self.queue.len()
    }

    fn enqueue(&mut self, url: String) {
        println!("enqueueing {}", url);
        self.queue.push(url.clone());

        /*ytd_rs::YoutubeDL::new(
            &std::path::PathBuf::from("videos/"),
            vec![
                ytd_rs::Arg::new("--write-info-json"),
                ytd_rs::Arg::new("--write-thumbnail"),
                ytd_rs::Arg::new("-o%(id)s"),
            ],
            &url,
        )
        .unwrap()
        .download()
        .unwrap();*/
    }

    fn pop_queue(&mut self) -> Result<crate::database::Job, anyhow::Error> {
        Err(anyhow::format_err!("Not implemented, sorry!"))
    }
}
