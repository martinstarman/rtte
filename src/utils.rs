use bevy::prelude::*;
use std::time::Duration;

pub fn timer_from_fps(fps: u8) -> Timer {
  Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
}
