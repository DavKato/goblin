use std::fs;
use std::io;

use crate::config;
use crate::local;

use config::Config;

pub fn init_local_env() -> Result<Config, io::Error> {
  // Create data folder
  let mut dir_path = local::get_path(local::Directory::Data, None);
  fs::create_dir(&dir_path)?;

  // Create config file's path
  dir_path.push(std::env::var("CONFIG_FILE_NAME").unwrap());

  // Create default setting and config file
  let config = Config::default();
  config.to_file()?;

  Ok(config)
}
