use serde::Deserialize;

#[derive(Deserialize)]
pub struct Animation {
  pub default_direction: String,
  pub frame_delay: i32,
  pub frame_height: i32,
  pub frame_width: i32,
  pub movements: Vec<String>,
  pub directions: Vec<String>,
}
