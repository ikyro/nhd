use requestty::{Answers, ErrorKind, Question};
use serde_json::{from_str, json, Value};
use std::{error, fs, io::Write, path::PathBuf};

pub struct Config {
  pub(crate) cf_clearance: String,
  pub(crate) user_agent: String,
  pub(crate) nhentai_path: PathBuf,
}

impl Config {
  pub fn new() -> Result<Self, Box<dyn error::Error>> {
    let nhentai_path = dirs::home_dir()
      .expect("Could not find home directory")
      .join(".nhentai")
      .join("config.json");
    let nhentai_path_parent = nhentai_path.parent().unwrap();
    let mut cf_clearance = String::new();
    let mut user_agent = String::new();

    if nhentai_path.is_file() {
      let file = fs::read_to_string(&nhentai_path).expect("Could not read config file");
      let file = from_str::<Value>(file.as_str()).expect("Could not parse config file");

      cf_clearance.push_str(file["cf_clearance"].as_str().unwrap());
      user_agent.push_str(file["user_agent"].as_str().unwrap());
    } else {
      if !nhentai_path_parent.is_dir() {
        fs::create_dir(&nhentai_path_parent)?;
      }

      let mut file = fs::File::create(&nhentai_path)?;
      let answers = Config::get_answers()?;
      let json = json!({
          "cf_clearance": answers.get("cf_clearance").unwrap().as_string(),
          "user_agent": answers.get("user_agent").unwrap().as_string(),
      });

      file.write_all(&json.to_string().as_bytes())?;
      cf_clearance.push_str(json["cf_clearance"].as_str().unwrap());
      user_agent.push_str(json["user_agent"].as_str().unwrap());
    }

    Ok(Self {
      cf_clearance,
      user_agent,
      nhentai_path,
    })
  }

  fn get_answers() -> Result<Answers, ErrorKind> {
    let questions = vec![
      Question::input("cf_clearance")
        .message("Enter your cf_clearance")
        .build(),
      Question::input("user_agent")
        .message("Enter your User-Agent")
        .build(),
    ];

    Ok(requestty::prompt(questions)?)
  }
}
