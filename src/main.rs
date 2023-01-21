use args::Args;
use clap::Parser;
use config::Config;
use nhentai::Nhentai;

mod args;
mod config;
mod nhentai;

fn main() {
    let cli = Args::parse();
    let Config {
        user_agent,
        csrftoken,
        nhentai_path,
    } = Config::new().unwrap();
    let doujin = Nhentai::new(cli.code, user_agent, csrftoken);

    println!("{}", doujin.get_title());

    doujin.build(nhentai_path);
}
