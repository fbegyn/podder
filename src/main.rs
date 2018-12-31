mod episode;
mod podcast;

use structopt::StructOpt;
use threadpool::ThreadPool;

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
    #[structopt(short = "b", long = "backlog", default_value = "5")]
    backlog: usize,
    #[structopt(short = "j", long = "threads", default_value = "3")]
    threads: usize,
}

// Program
fn main() -> Result<(), Box<std::error::Error>> {
    // Get CLI args and flags
    let args = Cli::from_args();
    let pool = ThreadPool::new(args.threads); // Create a worker pool

    // Get RSS channels from the arguments
    let feeds: Vec<rss::Channel> = args
        .urls
        .iter()
        .map(|x| {
            let mut t = rss::Channel::from_url(&x).expect("Failed to parse url");
            t.set_link(x as &str);
            t
        })
        .collect();

    let pods: Vec<podcast::Podcast> = feeds
        .iter()
        .map(move |f| {
            return podcast::Podcast {
                title: f.title().parse().expect("Failed to read podcast title"),
                url: f.link().parse().expect("Failed to read the link"),
                episodes: f.clone().into_items().len(),
            };
        })
        .collect();

    println!("{}\n", pods[0]);

    // TODO: make this iterate over all channels
    let eps = feeds[0].clone().into_items();
    let episodes = get_episodes(eps)?;

    // Start downloading the episodes
    for i in 0..args.backlog {
        let mut eps = episodes[i].clone();
        pool.execute(move || {
            eps.download();
        });
    }
    pool.join(); // Wait untill all the workers have finished
    Ok(())
}

// Creates episodes from an RSS feed
fn get_episodes(items: Vec<rss::Item>) -> Result<Vec<episode::Episode>, Box<std::error::Error>> {
    let mut episodes: Vec<episode::Episode> = Vec::new();
    for ep in items.iter() {
        episodes.push(episode::Episode::from_item(ep.clone())?);
    }
    Ok(episodes)
}
