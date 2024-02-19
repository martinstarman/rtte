use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game::Game;
use macroquad::{
  color::MAGENTA,
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

fn window_conf() -> Conf {
  Conf {
    window_title: "rtte".to_owned(),
    window_width: WINDOW_WIDTH,
    window_height: WINDOW_HEIGHT,
    window_resizable: false,
    high_dpi: true,
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

    // TODO: draw is not needed when all draws are as systems
    game.draw();
    game.update();

    next_frame().await
  }
}
