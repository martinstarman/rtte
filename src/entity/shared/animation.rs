use serde::Deserialize;

#[derive(Deserialize)]
pub struct Animation {
  pub frame_delay: i32,
  pub frame_height: i32,
  pub frame_row: i32,
  pub frame_width: i32,
}
