use args::Args;
use clap::Parser;
use config::Config;
use nhentai::Nhentai;
use std::{thread, time::Duration};

mod args;
mod config;
mod nhentai;

fn main() {
    let cli = Args::parse();
    let Config {
        user_agent,
        csrftoken,
    } = Config::new().unwrap();
    let doujin = Nhentai::new(cli.code, user_agent, csrftoken);

    let urls = doujin.get_pages_url().unwrap();

    thread::spawn(|| {
        urls.iter().for_each(|url| {
            doujin
                .client
                .get(url)
                .get(format!("https://nhentai.net/api/gallery/{}", code))
                .header(USER_AGENT, user_agent)
                .header(COOKIE, format!("csrftoken={}", csrftoken))
                .send()
                .unwrap()
                .json()
                .unwrap()
        })
    });
}
