use args::Args;
use clap::Parser;
use config::Config;
use nhentai::Nhentai;
use requestty::Question;
use std::path::{Path, PathBuf};

mod args;
mod config;
mod nhentai;

fn get_custom_path() -> PathBuf {
  let default_path = format!("{}", dirs::home_dir().unwrap().join(".nhentai").display());
  let ask_path = Question::input("path")
    .message("Where do you want to download the doujin?")
    .default(default_path)
    .validate(|path, _| {
      let path = Path::new(path);

      match path.exists() {
        true => Ok(()),
        false => Err("Path doesn't exist".to_string()),
      }
    })
    .build();
  let answer = requestty::prompt_one(ask_path).unwrap();
  let path = Path::new(answer.as_string().unwrap()).to_path_buf();

  path
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Args::parse();
  let Config {
    user_agent,
    cf_clearance,
    nhentai_path,
  } = Config::new()?;
  let doujin = Nhentai::new(cli.code, user_agent, cf_clearance).await;
  let path = match cli.path {
    true => get_custom_path(),
    false => nhentai_path.parent().unwrap().to_path_buf(),
  };

  if cli.force {
    doujin.build(path, cli.code).await;
  } else {
    let confirm = Question::confirm("confirm")
      .default(false)
      .message(format!("Do you want to download {} ?", doujin.get_title()))
      .build();

    let answer = requestty::prompt_one(confirm)?;

    if answer.as_bool().unwrap() {
      doujin.build(path, cli.code).await;
    }
  }

  Ok(())
}
