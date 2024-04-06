use serde::Deserialize;

#[derive(Deserialize)]
pub struct Animation {
  pub direction: String,
  pub frame_delay: i32,
  pub frame_height: i32,
  pub frame_width: i32,
  pub walk: Walk,
}

#[derive(Deserialize)]
pub struct Walk {
  pub frame_row: i32,
  pub directions: Vec<String>,
}
