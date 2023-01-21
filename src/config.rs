use dirs::home_dir;
use requestty::{Answers, ErrorKind, Question};
use serde_json::json;
use std::{
    error, fs,
    io::{Read, Write}, path::PathBuf,
};

pub struct Config {
    pub(crate) csrftoken: String,
    pub(crate) user_agent: String,
    pub(crate) nhentai_path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let nhentai_path = home_dir()
            .expect("Failed to get home directory path")
            .join(".nhentai");

        if nhentai_path.is_dir() {
            let mut file = fs::File::open(&nhentai_path.join("config.json"))?;
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)?;

            let buffer = json!(buffer);

            Ok(Self {
                csrftoken: buffer["csrftoken"].to_string(),
                user_agent: buffer["user_agent"].to_string(),
                nhentai_path,
            })
        } else {
            fs::create_dir(&nhentai_path)?;

            let answers = Config::get_answers()?;
            let mut file = fs::File::create(&nhentai_path.join("config.json"))?;
            let json = json!({
                "csrftoken": &answers.get("CSRFTOKEN").unwrap().as_string(),
                "user_agent": &answers.get("USER_AGENT").unwrap().as_string(),
            });

            file.write_all(&json.to_string().as_bytes())?;

            Ok(Self {
                csrftoken: json["csrftoken"].to_string(),
                user_agent: json["user_agent"].to_string(),
                nhentai_path,
            })
        }
    }

    fn get_answers() -> Result<Answers, ErrorKind> {
        let questions = vec![
            Question::input("csrftoken")
                .message("Enter your CSRF token")
                .build(),
            Question::input("user_agent")
                .message("Enter your User-Agent")
                .build(),
        ];

        Ok(requestty::prompt(questions)?)
    }
}
