use reqwest::{header, Client};
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct Episode {
    title: String,
    link: String,
    url: String,
    size: u64,
    release_date: String,
    downloaded: bool,
}

// Function that allows episodes to be printed
impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Title: {}\nRelease Date: {}\nLink: {}\nSize: {}\nURL: {}",
            self.title, self.release_date, self.link, self.size, self.url
        )
    }
}

impl Episode {
    // Getters and setters
    pub fn get_title(self) -> String {
        self.title
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn get_date(self) -> String {
        self.release_date
    }
    pub fn get_link(self) -> String {
        self.link
    }
    pub fn get_size(self) -> u64 {
        self.size
    }
    pub fn get_url(self) -> String {
        self.url
    }
    pub fn get_downloaded(self) -> bool {
        self.downloaded
    }
    pub fn set_downloaded(&mut self, val: bool) {
        self.downloaded = val;
    }

    // Get episode from rss::Item
    pub fn from_item(item: rss::Item) -> Result<Episode, Box<std::error::Error>> {
        Ok(Episode {
            title: item.title().expect("No title found").parse()?,
            link: item.link().expect("No link").parse()?,
            url: item.enclosure().expect("No enclosure:").url().parse()?,
            size: item
                .enclosure()
                .expect("No enclosure")
                .length()
                .parse()
                .expect("Unable to parse size"),
            release_date: item.pub_date().expect("No release date").parse()?,
            downloaded: false,
        })
    }

    // Download a given episode
    pub fn download(&mut self) {
        if self.downloaded {
            println!("Episode already downloaded")
        } else {
            let title = format!("{}.mp3", self.title.replace("/", " "));
            println!("Downloading: {}\n", title);
            let file = Path::new(&title);

            let client = Client::new();
            let mut request = client.get(&self.url);

            if file.exists() {
                let size = file.metadata().expect("Failed to read metadata").len() - 1;
                request = request.header(header::RANGE, format!("bytes={}-", size));
            }

            let mut resp = request.send().expect("Failed to send");

            let mut dst = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file)
                .expect("failed to open");

            let _copied = resp
                .copy_to(&mut dst)
                .expect("Something went wrong writing the file");

            self.set_downloaded(true);
        }
    }
}
