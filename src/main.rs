use rss::Channel;
use std::fmt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
#[structopt(
    name = "Podder",
    about = "An app to download your podcasts, fast and easy"
)]
struct Cli {
    #[structopt(
        help = "RSS feed urls for the podcasts we want to download",
        parse(from_str)
    )]
    urls: Vec<String>,
    #[structopt(short = "b", long = "backlog", default_value = "30")]
    backlog: i16,
}

struct Episode {
    title: String,
    link: String,
    url: String,
    size: i32,
    release_date: String,
}

impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Title: {}\nRelease Date: {}\nLink: {}\nSize: {}\nURL: {}\n\n",
            self.title, self.release_date, self.link, self.size, self.url
        )
    }
}

fn main() {
    let args = Cli::from_args();
    let feeds: Vec<rss::Channel> = args
        .urls
        .iter()
        .map(|x| Channel::from_url(x as &str).expect("Failed to parse url"))
        .collect();
    let eps = feeds[0].clone().into_items();
    let episodes = get_episodes(eps).expect("failed to get episodes");
    for i in 0..4 {
        println!("{}", episodes[i]);
    }
}

fn get_episodes(items: Vec<rss::Item>) -> std::io::Result<Vec<Episode>> {
    let mut episodes: Vec<Episode> = Vec::new();
    for ep in items.iter() {
        episodes.push(Episode {
            title: ep
                .title()
                .expect("Unable to parse title")
                .parse()
                .expect("Unable to parse title"),
            link: ep
                .link()
                .expect("No link")
                .parse()
                .expect("Unable to parse link"),
            url: ep
                .enclosure()
                .expect("No enclosure:")
                .url()
                .parse()
                .expect("Unable to parse url"),
            size: ep
                .enclosure()
                .expect("No enclosure")
                .length()
                .parse()
                .expect("Unable to parse size"),
            release_date: ep
                .pub_date()
                .expect("No release date")
                .parse()
                .expect("Unable to parse date"),
        });
    }
    Ok(episodes)
}
