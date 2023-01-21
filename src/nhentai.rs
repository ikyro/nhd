use reqwest::{
    blocking::Client,
    header::{COOKIE, USER_AGENT},
};
use serde_json::Value;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    thread,
};

#[derive(Debug)]
pub struct Nhentai {
    pub client: Client,
    doujin: Value,
}

impl Nhentai {
    pub fn new(code: i64, user_agent: String, csrftoken: String) -> Self {
        let client = Client::new();
        let doujin = client
            .get(format!("https://nhentai.net/api/gallery/{}", code))
            .header(USER_AGENT, user_agent)
            .header(COOKIE, format!("csrftoken={}", csrftoken))
            .send()
            .unwrap()
            .json()
            .unwrap();

        Self { doujin, client }
    }

    pub fn get_title(&self) -> String {
        match self.doujin["title"]["english"].is_null() {
            true => self.doujin["title"]["japanese"].to_string(),
            false => self.doujin["title"]["english"].to_string(),
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
                .map(|(i, page)| {
                    if page["t"] == "j" {
                        format!("https://i.nhentai.net/galleries/{}/{}.jpg", media_id, i + 1)
                    } else {
                        format!("https://i.nhentai.net/galleries/{}/{}.png", media_id, i + 1)
                    }
                })
                .collect(),
        )
    }

    pub fn build(&self, path: PathBuf) {
        let pages_url = self.get_pages_url().unwrap();
        let title = self
            .get_title()
            .replace(' ', "_")
            .replace(' ', "")
            .replace('"', "");
        let path = path.join(title);

        fs::create_dir(&path).unwrap();
        let handle = thread::spawn(move || {
            pages_url.iter().enumerate().for_each(|(i, url)| {
                let buffer = reqwest::blocking::get(url).unwrap().bytes().unwrap();
                let mut file = File::create(&path.join(format!(
                    "{}.{}",
                    i + 1,
                    url.split('.').last().unwrap()
                )))
                .unwrap();

                file.write_all(&buffer).unwrap();
            });
        });

        handle.join().unwrap();
    }
}
