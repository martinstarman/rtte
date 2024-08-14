use bevy::{prelude::*, window::PrimaryWindow};
use std::{collections::HashMap, time::Duration};

use crate::{camera::MainCamera, direction::Direction, movable::Movable};

const PLAYER_SPEED: f32 = 2.;

// TODO: split fields into components
#[derive(Component)]
pub struct Player {
  state: PlayerState,
  // TODO: direction: Direction,
}

#[derive(Component)]
pub struct PlayerAnimationConfig {
  pub fps: u8,
  pub frame_timer: Timer,
}

#[derive(Resource)]
pub struct PlayerAtlasConfig {
  map: HashMap<PlayerState, HashMap<Direction, Handle<TextureAtlasLayout>>>,
}

// TODO: consider
// pub struct PlayerState {
//   type: Idle,
//   atlas: HashMap<Direction, Handle<TextureAtlasLayout>>,
//   animation_config: ...
// }

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PlayerState {
  Idle = 1,
  Walk = 2,
}

impl PlayerAnimationConfig {
  // TODO: config
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
  mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
  let texture = asset_server.load("player_atlas.png");

  // TODO: config
  let tile_size = UVec2::new(256, 256);
  let mut atlas_config = HashMap::new();

  // TODO: config
  let directions = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
  ];

  let mut idle_config = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(i as u32 * 1024, 0));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 4, None, offset);
    let handle = atlases.add(atlas);
    idle_config.insert(direction.clone(), handle);
  }

  atlas_config.insert(PlayerState::Idle, idle_config);

  let mut walk_config = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(i as u32 * 1024, 1024));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 3, None, offset);
    let handle = atlases.add(atlas);
    walk_config.insert(direction.clone(), handle);
  }

  atlas_config.insert(PlayerState::Walk, walk_config);

  commands.insert_resource(PlayerAtlasConfig {
    map: atlas_config.clone(),
  });

  // TODO: PlayerBundle
  commands.spawn((
    Player {
      state: PlayerState::Idle,
    },
    Movable { path: vec![] },
    SpriteBundle {
      texture,
      ..default()
    },
    TextureAtlas::from(
      atlas_config
        .get(&PlayerState::Idle)
        .unwrap()
        .clone()
        .get(&Direction::South)
        .unwrap()
        .clone(),
    ),
    PlayerAnimationConfig::new(10),
  ));
}

pub fn player_animation(
  time: Res<Time>,
  mut animation_q: Query<(&Player, &mut TextureAtlas, &mut PlayerAnimationConfig)>,
) {
  for (player, mut atlas, mut animation_config) in &mut animation_q {
    animation_config.frame_timer.tick(time.delta());

    if animation_config.frame_timer.just_finished() {
      // TODO: config
      let frame_count = if player.state == PlayerState::Idle {
        13
      } else {
        8
      };
      atlas.index = (atlas.index + 1) % frame_count;

      animation_config.frame_timer = PlayerAnimationConfig::timer_from_fps(animation_config.fps);
    }
  }
}

pub fn player_direction(
  mut player_atlas_q: Query<(&Player, &Movable, &Transform, &mut TextureAtlas)>,
  atlas_config: Res<PlayerAtlasConfig>,
) {
  for (player, movable, transform, mut atlas) in &mut player_atlas_q {
    if movable.path.len() > 0 {
      let angle =
        (movable.path[0] - Vec2::new(transform.translation.x, transform.translation.y)).to_angle();
      let direction = Direction::try_from(angle).unwrap();

      atlas.layout = atlas_config
        .map
        .get(&player.state)
        .unwrap()
        .clone()
        .get(&direction)
        .unwrap()
        .clone();
    }
  }
}

pub fn player_update_path(
  mut player_q: Query<&mut Movable, With<Player>>,
  buttons: Res<ButtonInput<MouseButton>>,
  windows_q: Query<&Window, With<PrimaryWindow>>,
  camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
  if buttons.just_pressed(MouseButton::Left) {
    let window = windows_q.single();

    if let Some(cursor_position) = window.cursor_position() {
      let (camera, global_transform) = camera_q.single();

      if let Some(position) = camera.viewport_to_world_2d(global_transform, cursor_position) {
        for mut movable in &mut player_q {
          movable.path.push(position);
        }
      }
    }
  }
}

pub fn player_follow_path(
  mut player_transform_q: Query<(&mut Movable, &mut Transform), With<Player>>,
) {
  for (mut movable, mut transform) in &mut player_transform_q {
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

pub fn player_state(mut player_q: Query<(&mut Player, &Movable), Changed<Movable>>) {
  for (mut player, movable) in &mut player_q {
    if movable.path.len() == 0 && player.state != PlayerState::Idle {
      player.state = PlayerState::Idle; // TODO: update atlas layout
    }

    if movable.path.len() > 0 && player.state != PlayerState::Walk {
      player.state = PlayerState::Walk;
    }
  }
}
