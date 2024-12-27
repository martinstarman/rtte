use bevy::{math::bounding::Aabb2d, prelude::*};
use std::time::Duration;

pub fn timer_from_fps(fps: u8) -> Timer {
  Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
}

pub fn contains(aabb: Aabb2d, vec: Vec2) -> bool {
  aabb.min.x <= vec.x && aabb.min.y <= vec.y && aabb.max.x >= vec.x && aabb.max.y >= vec.y
}
