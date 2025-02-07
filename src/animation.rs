use bevy::{math::CompassOctant, prelude::*};
use std::collections::HashMap;

#[derive(Resource)]
pub struct Animation<T> {
  pub frame_timer: Timer,
  pub atlas_config: HashMap<T, AnimationAtlasConfig>,
}

#[derive(Clone)]
pub struct AnimationAtlasConfig {
  pub fps: u8,
  pub frame_count: u8,
  pub layouts: HashMap<CompassOctant, Handle<TextureAtlasLayout>>,
}
