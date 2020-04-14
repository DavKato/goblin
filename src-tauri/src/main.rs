#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use serde::Serialize;
use std::sync::Arc;
use std::{env, io};

use config::Config;
use goblib::{cmd, config, init, local};

fn main() {
  env::set_var("CONFIG_FILE_NAME", "gob.config.json");

  let config = match Config::from_file() {
    Ok(cf) => cf,
    Err(err) => match err.kind() {
      io::ErrorKind::NotFound => init::init_local_env().expect("Error initializing app"),
      e => panic!("Error reading config file: {:?}", e),
    },
  };
  let config = Arc::new(config);
  let cf_c1 = Arc::clone(&config);

  tauri::AppBuilder::new()
    // .splashscreen_html(r#"<div style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);">Loading...</div>"#)
    .setup(move |webview, _source| {
      // let data = ConfigData {
      //   project_name: String::from("Sample"),
      //   contents: ["ブログ", "タグ", "ピックアップタグ"],
      // };
      //////
      // Test field
      //////
      let handle = webview.handle();
      tauri::event::emit(
        &handle,
        String::from("initConfig"),
        Some(serde_json::to_string(&cf_c1).unwrap()),
      );
    })
    .invoke_handler(move |_webview, arg| {
      use cmd::Cmd::*;
      let cf_c2 = Arc::clone(&config);
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            CreateProject {
              project_name,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || match local::create_project(&project_name) {
                Ok(_) => {
                  let mut current_project = cf_c2.current_project.lock().unwrap();
                  *current_project = project_name;
                  Ok(r#"{message: "プロジェクトを作成しました。"}"#.to_string())
                }
                Err(_) => Err(tauri::Error::from("プロジェクトの作成に失敗しました。")),
              },
              callback,
              error,
            ),

            GetError { callback, error } => tauri::execute_promise(
              _webview,
              move || {
                println!("{}", cf_c2.current_project.lock().unwrap());
                Err(tauri::Error::from("。エラーなり!!。"))
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
