use bevy::{prelude::*, window::PrimaryWindow};
use std::collections::HashMap;

use crate::{
  camera::MainCamera,
  direction::{Direction, Directions},
  movable::Movable,
  utils::timer_from_fps,
};

const PLAYER_SPEED: f32 = 2.;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct PlayerAnimationConfig {
  pub fps: u8,
  pub frame_timer: Timer,
}

#[derive(Resource)]
pub struct PlayerAtlasConfig {
  map: HashMap<PlayerStates, HashMap<Directions, Handle<TextureAtlasLayout>>>,
}

#[derive(Component)]
pub struct PlayerState {
  pub value: PlayerStates,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PlayerStates {
  Idle = 1,
  Walk = 2,
}

pub fn player_setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
  let texture = asset_server.load("player_atlas.png");

  // TODO: config
  let tile_size = UVec2::new(256, 256);
  let mut atlas_config = HashMap::new();

  // TODO: config
  let directions = [
    Directions::North,
    Directions::NorthEast,
    Directions::East,
    Directions::SouthEast,
    Directions::South,
    Directions::SouthWest,
    Directions::West,
    Directions::NorthWest,
  ];

  let mut idle_config = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(i as u32 * 1024, 0));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 4, None, offset);
    let handle = atlases.add(atlas);
    idle_config.insert(direction.clone(), handle);
  }

  atlas_config.insert(PlayerStates::Idle, idle_config);

  let mut walk_config = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(i as u32 * 1024, 1024));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, offset);
    let handle = atlases.add(atlas);
    walk_config.insert(direction.clone(), handle);
  }

  atlas_config.insert(PlayerStates::Walk, walk_config);

  commands.insert_resource(PlayerAtlasConfig {
    map: atlas_config.clone(),
  });

  // TODO: PlayerBundle
  commands.spawn((
    Player {},
    Movable { path: vec![] },
    PlayerState {
      value: PlayerStates::Idle,
    },
    Direction {
      value: Directions::South,
    },
    SpriteBundle {
      texture,
      ..default()
    },
    PlayerAnimationConfig {
      fps: 10,
      frame_timer: timer_from_fps(10),
    },
    TextureAtlas::from(
      atlas_config
        .get(&PlayerStates::Idle)
        .unwrap()
        .clone()
        .get(&Directions::South)
        .unwrap()
        .clone(),
    ),
  ));
}

pub fn player_animation(
  mut query: Query<(&PlayerState, &mut TextureAtlas, &mut PlayerAnimationConfig), With<Player>>,
  time: Res<Time>,
) {
  for (player_state, mut atlas, mut animation_config) in &mut query {
    animation_config.frame_timer.tick(time.delta());

    if animation_config.frame_timer.just_finished() {
      // TODO: config
      let frame_count = if player_state.value == PlayerStates::Idle {
        13
      } else {
        8
      };
      atlas.index = (atlas.index + 1) % frame_count;

      animation_config.frame_timer = timer_from_fps(animation_config.fps);
    }
  }
}

pub fn player_atlas_layout(
  mut query: Query<
    (&PlayerState, &Direction, &mut TextureAtlas),
    (With<Player>, Or<(Changed<Direction>, Changed<PlayerState>)>),
  >,
  atlas_config: Res<PlayerAtlasConfig>,
) {
  for (player_state, direction, mut atlas) in &mut query {
    atlas.layout = atlas_config
      .map
      .get(&player_state.value)
      .unwrap()
      .clone()
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
  windows_q: Query<&Window, With<PrimaryWindow>>,
  camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
  if buttons.just_pressed(MouseButton::Left) {
    let window = windows_q.single();

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

      if transform.translation.distance(next) <= 1. {
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
