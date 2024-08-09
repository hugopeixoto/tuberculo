use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: String,
}

pub struct Database {
    pub videos: Vec<Video>,
    pub queue: Vec<String>,
}

impl Database {
    pub fn search(&self, _term: &str) -> Vec<Video> {
        self.videos.clone()
    }

    pub fn new() -> Self {
        let mut videos = vec![
            Video {
                id: "21312".into(),
                title: "Kendrick Lamar - Not Like Us".into(),
                description: "Banger".into(),
            },
            Video {
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

    pub fn enqueue(&mut self, url: String) {
        println!("enqueueing {}", url);
        self.queue.push(url.clone());

        let download = ytd_rs::YoutubeDL::new(
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
        .unwrap();
    }
}
