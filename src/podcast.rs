use std::fmt;

#[derive(Clone)]
pub struct Podcast {
    pub title: String,
    pub url: String,
    pub episodes: usize,
}

impl fmt::Display for Podcast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Title: {}\nURL: {}\n# episodes: {}",
            self.title, self.url, self.episodes
        )
    }
}
