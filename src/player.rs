use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerAnimationConfig {
  pub fps: u8,
  pub frame_timer: Timer,
}

impl PlayerAnimationConfig {
  fn new(fps: u8) -> Self {
    Self {
      fps,
      frame_timer: Self::timer_from_fps(fps),
    }
  }

  fn timer_from_fps(fps: u8) -> Timer {
    Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
  }
}

pub fn player_setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
  let texture_handle = asset_server.load("test.png");
  let texture_atlas = TextureAtlasLayout::from_grid(UVec2 { x: 256, y: 256 }, 4, 3, None, None);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  commands.spawn((
    Player,
    SpriteBundle {
      texture: texture_handle,
      ..default()
    },
    TextureAtlas::from(texture_atlas_handle),
    PlayerAnimationConfig::new(10),
  ));
}

pub fn player_animation(
  time: Res<Time>,
  mut texture_atlas_q: Query<(&mut TextureAtlas, &mut PlayerAnimationConfig), With<Player>>,
) {
  for (mut texture_atlas, mut player_animation_config) in &mut texture_atlas_q {
    player_animation_config.frame_timer.tick(time.delta());

    if player_animation_config.frame_timer.just_finished() {
      texture_atlas.index = (texture_atlas.index + 1) % 8;
      player_animation_config.frame_timer =
        PlayerAnimationConfig::timer_from_fps(player_animation_config.fps);
    }
  }
}
