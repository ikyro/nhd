use futures::future::join_all;
use reqwest::{
  header::{COOKIE, USER_AGENT},
  Client,
};
use serde_json::Value;
use std::{
  fs::{create_dir, File},
  io::Write,
  path::PathBuf,
  time::Instant,
};

#[derive(Debug)]
pub struct Nhentai {
  doujin: Value,
  client: Client,
}

impl Nhentai {
  pub async fn new(code: i64, user_agent: String, cf_clearance: String) -> Self {
    let client = Client::new();

    let doujin = client
      .get(format!("https://nhentai.net/api/gallery/{}", code))
      .header(USER_AGENT, user_agent)
      .header(COOKIE, format!("cf_clearance={}", cf_clearance))
      .send()
      .await
      .expect("Failed to send request")
      .json()
      .await
      .expect("Failed to parse response");

    Self { doujin, client }
  }

  pub fn get_title(&self) -> String {
    if !self.doujin["title"]["pretty"].is_null() {
      self.doujin["title"]["pretty"].to_string()
    } else if !self.doujin["title"]["english"].is_null() {
      self.doujin["title"]["english"].to_string()
    } else {
      self.doujin["title"]["japanese"].to_string()
    }
  }

  pub fn get_pages_url(&self) -> Option<Vec<String>> {
    let media_id = match self.doujin["media_id"].is_number() {
      true => self.doujin["media_id"].as_i64()?,
      false => self.doujin["media_id"].as_str()?.parse().unwrap(),
    };

    Some(
      self.doujin["images"]["pages"]
        .as_array()?
        .iter()
        .enumerate()
        .map(|(i, page)| match page["t"].as_str() {
          Some("j") => {
            format!("https://i.nhentai.net/galleries/{}/{}.jpg", media_id, i + 1)
          }
          Some("p") => {
            format!("https://i.nhentai.net/galleries/{}/{}.png", media_id, i + 1)
          }
          _ => format!("https://i.nhentai.net/galleries/{}/{}.jpg", media_id, i + 1),
        })
        .collect(),
    )
  }

  pub async fn build(&self, path: PathBuf, code: i64) {
    let pages_url = self.get_pages_url().unwrap();
    let path = path.join(code.to_string());
    let lenght = &pages_url.len();

    create_dir(&path).expect(format!("Failed to create directory {}", path.display()).as_str());

    let start = Instant::now();
    let tasks = pages_url.into_iter().enumerate().map(|(i, url)| {
      let path = path.clone();
      let client = self.client.clone();

      tokio::spawn(async move {
        let buffer = client
          .get(&url)
          .send()
          .await
          .expect("Failed to request image")
          .bytes()
          .await
          .expect("Failed to get image bytes");

        let mut file =
          File::create(path.join(format!("{}.{}", i + 1, url.split('.').last().unwrap())))
            .expect("Failed to create file");

        file.write_all(&buffer).unwrap();
      })
    });

    join_all(tasks).await;
    println!(
      "{} pages downloaded in {:?} to {}",
      lenght,
      start.elapsed(),
      path.display()
    )
  }
}
