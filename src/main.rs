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
pub mod resource;
pub mod system;

fn window_conf() -> Conf {
  Conf {
    window_title: "rtte".to_owned(),
    window_width: WINDOW_WIDTH,
    window_height: WINDOW_HEIGHT,
    window_resizable: false,
    high_dpi: true,
    fullscreen: true,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let mut game = Game::new().await;

  loop {
    clear_background(MAGENTA);
    game.update();
    next_frame().await
  }
}
