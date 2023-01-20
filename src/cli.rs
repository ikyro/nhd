use crate::args::Args;
use clap::Parser;
use dirs::home_dir;
use requestty::{Answers, ErrorKind, Question};
use serde_json::json;
use std::{
    error,
    fs::{create_dir, File},
    io::{Read, Write},
};

#[derive(Debug)]
pub struct Cli {
    pub args: Args,
    pub config: serde_json::Value,
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

impl Cli {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let nhenta_path = home_dir()
            .expect("Failed to get home directory")
            .join(".nhentai");

        if nhenta_path.is_dir() {
            let mut file = File::open(&nhenta_path.join("config.json"))?;
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)?;

            Ok(Self {
                args: Args::parse(),
                config: json!(buffer),
            })
        } else {
            create_dir(&nhenta_path)?;

            let answers = get_answers()?;
            let mut file = File::create(&nhenta_path.join("config.json"))?;
            let json = json!({
                "csrftoken": &answers.get("CSRFTOKEN").unwrap().as_string(),
                "user_agent": &answers.get("USER_AGENT").unwrap().as_string(),
            });

            file.write_all(&json.to_string().as_bytes())?;

            Ok(Self {
                args: Args::parse(),
                config: json,
            })
        }
    }
}
