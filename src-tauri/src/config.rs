use crate::local;
use serde::{Deserialize, Serialize};
// use std::cell::RefCell;
use std::sync::Mutex;
use std::{fs, io};

#[derive(Serialize, Deserialize)]
pub enum Theme {
  Dark,
  Blue,
}

#[derive(Serialize, Deserialize)]
pub enum ExportPath {
  Documents,
  Desktop,
}

#[derive(Serialize, Deserialize)]
pub enum ExportFormat {
  Json,
  Markdown,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub current_project: Mutex<String>,
  pub theme: Mutex<Theme>,
  pub export_path: Mutex<ExportPath>,
  pub export_format: Mutex<ExportFormat>,
}

impl Config {
  pub fn to_file(&self) -> Result<(), io::Error> {
    let path = local::get_config_path();
    let data = serde_json::to_string(self).unwrap();
    fs::write(path, data)
  }

  pub fn from_file() -> Result<Self, io::Error> {
    let path = local::get_config_path();
    let file = fs::read(path)?;
    let config: Config = serde_json::from_slice(&file)?;
    Ok(config)
  }

  pub fn default() -> Self {
    Self {
      current_project: Mutex::new("".to_string()),
      theme: Mutex::new(Theme::Dark),
      export_path: Mutex::new(ExportPath::Documents),
      export_format: Mutex::new(ExportFormat::Json),
    }
  }
}
