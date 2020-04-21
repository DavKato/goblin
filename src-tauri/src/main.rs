#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

// use serde::Serialize;
use rusqlite::params;
use std::collections::HashMap;
use std::sync::Arc;

use goblib::*;

fn main() {
  dotenv::dotenv().ok();
  let config = config::Config::new();
  let config = Arc::new(config);
  let cf_c1 = Arc::clone(&config);

  let db = db::DB::new();

  tauri::AppBuilder::new()
    // .splashscreen_html(r#"<div style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);">Loading...</div>"#)
    .setup(move |webview, _source| {
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
            } => {
              let operation = "プロジェクトの作成";
              tauri::execute_promise(
                _webview,
                move || match local::create_project(&project_name) {
                  Ok(_) => {
                    let mut config = cf_c2.lock().unwrap();
                    config.current_project = project_name;
                    match config.to_file() {
                      Ok(_) => Ok(serde_json::to_string(operation).unwrap()),

                      Err(err) => Err(tauri::Error::from(msg::Error::from(operation, err.to_string()))),
                    }
                  }
                  Err(err) => Err(tauri::Error::from(msg::Error::from(operation, err.to_string()))),
                },
                callback,
                error,
              );
            }

            GetError { callback, error } => tauri::execute_promise(
              _webview,
              move || {
                let operation = "エラー送信";
                let err = "エラー！エラー！".to_string();
                Err(tauri::Error::from(msg::Error::from(operation, err)))
              },
              callback,
              error,
            ),

            TestQuery => {
              db.conn.execute("DROP TABLE IF EXISTS cities", params![]).unwrap();

              db.conn
                .execute(
                  "
                    CREATE TABLE cities (
                      id INTEGER PRIMARY KEY NOT NULL,
                      name TEXT NOT NULL,
                      temp TEXT,
                      popularity TEXT
                    );
                  ",
                  params![],
                )
                .expect("Failed Creating Table");

              db.conn
                .execute(
                  "
                    INSERT INTO cities (name, temp, popularity)
                    VALUES('San Jose', '18', 'low'),
                    ('Domingo', '29', 'high'),
                    ('Paparacci', '-20', 'medium');
              ",
                  params![],
                )
                .expect("Failed Inserting.");

              let mut stmt = db.conn.prepare("SELECT * FROM cities").unwrap();
              let res = stmt
                .query_map(params![], |row| {
                  let columns = row.column_names();
                  let mut columns = columns.iter();

                  let mut map = HashMap::new();
                  map.insert(
                    columns.next().unwrap().to_string(),
                    row.get::<_, u8>(0).unwrap().to_string(),
                  );

                  for (i, c) in columns.enumerate() {
                    map.insert(c.to_string(), row.get(i + 1).unwrap());
                  }
                  Ok(map)
                })
                .unwrap();

              for line in res {
                println!("{:?}", line.unwrap());
              }
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
