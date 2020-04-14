use std::fs;
use std::io;
use std::path::PathBuf;

use tauri::api::path;

pub enum Directory {
  Data,
  Document,
  Desktop,
}

pub fn get_path(dir: Directory, path: Option<&str>) -> PathBuf {
  let mut dir_path = match dir {
    Directory::Data => data_dir(),
    Directory::Document => document_dir(),
    Directory::Desktop => desktop_dir(),
  };
  if let Some(p) = path {
    dir_path.push(p);
  };
  dir_path
}

fn data_dir() -> PathBuf {
  let mut dir = path::local_data_dir().unwrap();
  let app_name: String = env!("CARGO_PKG_NAME").to_string();
  dir.push(app_name);
  dir
}
fn document_dir() -> PathBuf {
  path::document_dir().unwrap()
}
fn desktop_dir() -> PathBuf {
  path::desktop_dir().unwrap()
}

pub fn get_config_path() -> PathBuf {
  let file_name = std::env::var("CONFIG_FILE_NAME").unwrap();
  let path: Option<&str> = Some(&file_name);
  get_path(Directory::Data, path)
}

pub fn create_project(project_name: &str) -> io::Result<()> {
  let path = get_path(Directory::Data, Some(project_name));
  fs::create_dir_all(path)
}
