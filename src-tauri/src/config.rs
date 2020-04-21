use crate::local::{get_path, Directory};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug)]
pub enum Theme {
  Dark,
  Blue,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExportPath {
  Documents,
  Desktop,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExportFormat {
  Json,
  Markdown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub current_project: String,
  pub theme: Theme,
  pub export_path: ExportPath,
  pub export_format: ExportFormat,
}

impl Config {
  pub fn new() -> Mutex<Self> {
    let config = match from_file() {
      Ok(c) => c,
      Err(err) => match err.kind() {
        io::ErrorKind::NotFound => init().unwrap_or_else(|err| panic!("Error initializing app. error: {}", err)),
        err => panic!("Error reading config file: {:?}", err),
      },
    };
    Mutex::new(config)
  }

  pub fn to_file(&self) -> Result<(), io::Error> {
    let path = get_config_path();
    let data = serde_json::to_string(self).unwrap();
    fs::write(path, data)
  }
}

fn get_config_path() -> PathBuf {
  let file_name = std::env::var("CONFIG_FILE_NAME").unwrap();
  let path: Option<&str> = Some(&file_name);
  get_path(Directory::Data, path)
}

fn default() -> Config {
  Config {
    current_project: "".to_string(),
    theme: Theme::Dark,
    export_path: ExportPath::Documents,
    export_format: ExportFormat::Json,
  }
}

fn from_file() -> Result<Config, io::Error> {
  let path = get_config_path();
  let file = fs::read(path)?;
  let config: Config = serde_json::from_slice(&file)?;
  Ok(config)
}

fn init() -> Result<Config, io::Error> {
  // Create data folder
  let dir_path = get_path(Directory::Data, None);
  if let Err(_) = fs::create_dir(&dir_path) {
    println!("{:?} exists.", &dir_path)
  };

  // Create default setting and config file
  let config = default();
  config.to_file()?;

  Ok(config)
}
