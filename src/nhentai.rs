use reqwest::{
    blocking::Client,
    header::{COOKIE, USER_AGENT},
};
use serde_json::Value;

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
}
