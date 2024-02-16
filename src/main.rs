use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game::Game;
use macroquad::{
  color::MAGENTA,
  input::set_cursor_grab,
  window::{clear_background, next_frame, Conf},
};

pub mod component;
pub mod constants;
pub mod entity;
pub mod event;
pub mod game;
pub mod mission;
pub mod point;
pub mod resource;
pub mod system;

// TODO
fn window_conf() -> Conf {
  Conf {
    window_title: "rtte".to_owned(),
    window_width: WINDOW_WIDTH,
    window_height: WINDOW_HEIGHT,
    window_resizable: false,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let mut game = Game::new().await;

  // @see https://github.com/not-fl3/macroquad/issues/557
  // set_cursor_grab(true);

  loop {
    clear_background(MAGENTA);

    game.update();
    game.draw();

    next_frame().await
  }
}
