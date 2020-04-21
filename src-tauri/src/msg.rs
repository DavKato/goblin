use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Error<'a> {
  pub operation: &'a str,
  pub err: String,
}

impl Error<'_> {
  // pub fn from<'a>(operation: &'a str, err: String) -> Error<'a> {
  //   Error { operation, err }
  // }

  // pub fn to_string(&self) -> String {
  //   format!(
  //     "{{ \\\"operation\\\": \\\"{}\\\", \\\"err\\\": \\\"{}\\\" }}",
  //     self.operation, self.err
  //   )
  // }

  pub fn from<'a>(operation: &'a str, err: String) -> String {
    format!(
      "{{ \\\"operation\\\": \\\"{}\\\", \\\"err\\\": \\\"{}\\\" }}",
      operation, err
    )
  }
}
