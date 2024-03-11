use macroquad::{color::WHITE, text::draw_text, time::get_fps};

use crate::constants::DEBUG;

pub fn draw_fps() {
  if DEBUG {
    draw_text(format!("{}", get_fps()).as_str(), 0., 16., 32., WHITE)
  }
}
