use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  CreateProject {
    project_name: String,
    callback: String,
    error: String,
  },
  GetError {
    callback: String,
    error: String,
  },
  TestQuery,
}
