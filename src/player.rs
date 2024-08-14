use bevy::{prelude::*, window::PrimaryWindow};
use std::collections::HashMap;

use crate::{
  camera::MainCamera,
  direction::{Direction, Directions},
  movable::Movable,
  utils::timer_from_fps,
};

const PLAYER_SPEED: f32 = 2.;
const PLAYER_MIN_MOVABLE_DISTANCE: f32 = 1.;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerAnimation {
  pub frame_timer: Timer,
  pub atlas_config: HashMap<PlayerStates, AtlasConfig>,
}

#[derive(Clone)]
pub struct AtlasConfig {
  fps: u8,
  frame_count: u8,
  layouts: HashMap<Directions, Handle<TextureAtlasLayout>>,
}

#[derive(Component, Default)]
pub struct PlayerState {
  pub value: PlayerStates,
}

#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub enum PlayerStates {
  #[default]
  Idle = 1,
  Walk = 2,
}

pub fn player_setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
  let mut atlas_config = HashMap::new();
  let texture = asset_server.load("player_atlas.png");
  let tile_size = UVec2::new(256, 256);
  let fps = 10;
  let directions = vec![
    Directions::North,
    Directions::NorthEast,
    Directions::East,
    Directions::SouthEast,
    Directions::South,
    Directions::SouthWest,
    Directions::West,
    Directions::NorthWest,
  ];

  let mut layouts = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(i as u32 * 1024, 0));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 4, None, offset);
    let handle = atlases.add(atlas);
    layouts.insert(direction.clone(), handle);
  }

  let config = AtlasConfig {
    fps,
    frame_count: 13,
    layouts,
  };

  atlas_config.insert(PlayerStates::Idle, config);

  let mut layouts = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(i as u32 * 1024, 1024));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, offset);
    let handle = atlases.add(atlas);
    layouts.insert(direction.clone(), handle);
  }

  let config = AtlasConfig {
    fps,
    frame_count: 8,
    layouts,
  };

  atlas_config.insert(PlayerStates::Walk, config);

  let default_fps = atlas_config.get(&PlayerStates::Idle).unwrap().fps;
  let default_layout = atlas_config
    .clone()
    .get(&PlayerStates::Idle)
    .unwrap()
    .layouts
    .get(&Directions::South)
    .unwrap()
    .clone();

  commands.insert_resource(PlayerAnimation {
    frame_timer: timer_from_fps(default_fps),
    atlas_config,
  });

  commands.spawn((
    Player,
    Movable::default(),
    PlayerState::default(),
    Direction::default(),
    SpriteBundle {
      texture,
      ..default()
    },
    TextureAtlas::from(default_layout),
  ));
}

pub fn player_animation(
  mut query: Query<(&PlayerState, &mut TextureAtlas), With<Player>>,
  mut animation: ResMut<PlayerAnimation>,
  time: Res<Time>,
) {
  for (player_state, mut atlas) in &mut query {
    animation.frame_timer.tick(time.delta());

    if animation.frame_timer.just_finished() {
      let atlas_config = animation.atlas_config.get(&player_state.value).unwrap();
      atlas.index = (atlas.index + 1) % atlas_config.frame_count as usize;
      animation.frame_timer = timer_from_fps(atlas_config.fps);
    }
  }
}

pub fn player_atlas_layout(
  mut query: Query<
    (&PlayerState, &Direction, &mut TextureAtlas),
    (With<Player>, Or<(Changed<Direction>, Changed<PlayerState>)>),
  >,
  animation: Res<PlayerAnimation>,
) {
  for (player_state, direction, mut atlas) in &mut query {
    atlas.layout = animation
      .atlas_config
      .get(&player_state.value)
      .unwrap()
      .layouts
      .get(&direction.value)
      .unwrap()
      .clone();
  }
}

pub fn player_direction(mut query: Query<(&Movable, &mut Direction, &Transform), With<Player>>) {
  for (movable, mut direction, transform) in &mut query {
    if movable.path.len() > 0 {
      let angle =
        (movable.path[0] - Vec2::new(transform.translation.x, transform.translation.y)).to_angle();
      direction.value = Directions::try_from(angle).unwrap();
    }
  }
}

pub fn player_path(
  mut query: Query<&mut Movable, With<Player>>,
  buttons: Res<ButtonInput<MouseButton>>,
  windows: Query<&Window, With<PrimaryWindow>>,
  camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
  if buttons.just_pressed(MouseButton::Left) {
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
      let (camera, global_transform) = camera_q.single();

      if let Some(position) = camera.viewport_to_world_2d(global_transform, cursor_position) {
        for mut movable in &mut query {
          movable.path.push(position);
        }
      }
    }
  }
}

pub fn player_follow_path(mut query: Query<(&mut Movable, &mut Transform), With<Player>>) {
  for (mut movable, mut transform) in &mut query {
    if movable.path.len() > 0 {
      let curr = transform.translation;
      let next = Vec3::new(movable.path[0].x, movable.path[0].y, 0.);
      let norm = (next - curr).normalize();

      transform.translation = curr + norm * PLAYER_SPEED;

      if transform.translation.distance(next) <= PLAYER_MIN_MOVABLE_DISTANCE {
        movable.path.remove(0);
      }
    }
  }
}

pub fn player_state(mut query: Query<(&mut PlayerState, &Movable), Changed<Movable>>) {
  for (mut player_state, movable) in &mut query {
    if movable.path.len() == 0 && player_state.value != PlayerStates::Idle {
      player_state.value = PlayerStates::Idle;
    }

    if movable.path.len() > 0 && player_state.value != PlayerStates::Walk {
      player_state.value = PlayerStates::Walk;
    }
  }
}
