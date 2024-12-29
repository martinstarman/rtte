use bevy::{math::bounding::Aabb2d, prelude::*};
use std::collections::HashMap;
use vleue_navigator::prelude::PrimitiveObstacle;

use crate::{
  animation::{Animation, AnimationAtlasConfig},
  bounding_box::BoundingBox,
  direction::{Direction, Directions},
  line_of_sight::{LineOfSight, LineOfSightShift, LINE_OF_SIGHT_VERTICES},
  movable::{Movable, PathItem, Speed::Slow},
  selectable::Selectable,
  utils::timer_from_fps,
  ysort::YSort,
};

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct EnemyState {
  pub value: EnemyStates,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EnemyStates {
  #[default]
  Idle = 1,
  Walk = 2,
  Run = 3,
}

pub fn enemy_setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
  let mut atlas_config = HashMap::new();
  let image = asset_server.load("enemy/export.png");
  let tile_size = UVec2::new(16, 32);
  let directions = vec![
    Directions::East,
    Directions::NorthEast,
    Directions::North,
    Directions::NorthWest,
    Directions::West,
    Directions::SouthWest,
    Directions::South,
    Directions::SouthEast,
  ];

  let mut layouts = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(0, i as u32 * tile_size.y));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 1, None, offset);
    let handle = atlases.add(atlas);
    layouts.insert(direction.clone(), handle);
  }

  let config = AnimationAtlasConfig {
    fps: 10,
    frame_count: 4,
    layouts,
  };

  atlas_config.insert(EnemyStates::Idle, config);

  let mut layouts = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(0, (i as u32 * tile_size.y) + 256));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 1, None, offset);
    let handle = atlases.add(atlas);
    layouts.insert(direction.clone(), handle);
  }

  let config = AnimationAtlasConfig {
    fps: 10,
    frame_count: 4,
    layouts,
  };

  atlas_config.insert(EnemyStates::Walk, config);

  let mut layouts = HashMap::new();

  for (i, direction) in directions.iter().enumerate() {
    let offset = Some(UVec2::new(0, (i as u32 * tile_size.y) + 512));
    let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 1, None, offset);
    let handle = atlases.add(atlas);
    layouts.insert(direction.clone(), handle);
  }

  let config = AnimationAtlasConfig {
    fps: 10,
    frame_count: 4,
    layouts,
  };

  atlas_config.insert(EnemyStates::Run, config);

  let default_fps = atlas_config.get(&EnemyStates::Idle).unwrap().fps;
  let default_layout = atlas_config
    .clone()
    .get(&EnemyStates::Idle)
    .unwrap()
    .layouts
    .get(&Directions::South)
    .unwrap()
    .clone();

  commands.insert_resource(Animation {
    frame_timer: timer_from_fps(default_fps),
    atlas_config,
  });

  let path = vec![
    PathItem {
      position: Vec2::new(200., 150.),
      speed: Slow,
    },
    PathItem {
      position: Vec2::new(200., -100.),
      speed: Slow,
    },
    PathItem {
      position: Vec2::new(0., 100.),
      speed: Slow,
    },
  ];

  commands
    .spawn((
      Enemy,
      Movable {
        path: path.clone(),
        default_path: path.clone(),
      },
      EnemyState::default(),
      Direction::default(),
      Sprite {
        image,
        texture_atlas: Some(TextureAtlas::from(default_layout)),
        ..default()
      },
      Transform::from_xyz(0., 100., 0.),
      YSort { height: 32 },
      LineOfSight {
        looking_at: Vec2::X.normalize(),
        offset: 0,
        shift: LineOfSightShift::Left,
        polygon: Polygon::new([Vec2::ZERO; LINE_OF_SIGHT_VERTICES]),
      },
      Selectable::default(),
    ))
    .with_children(|parent| {
      parent.spawn((
        Transform::from_translation(Vec3::new(0., -12., 0.)),
        PrimitiveObstacle::Rectangle(Rectangle::new(16., 8.)),
      ));

      parent.spawn(BoundingBox {
        value: Aabb2d::new(Vec2::ZERO, Vec2::new(8., 16.)),
      });
    })
    .observe(enemy_select::<Pointer<Up>>());
}

fn enemy_select<E>() -> impl Fn(Trigger<E>, Query<(Entity, &mut Selectable), With<Enemy>>) {
  move |event, mut query| {
    for (entity, mut selectable) in &mut query {
      if entity == event.entity() {
        selectable.selected = !selectable.selected;
      } else {
        selectable.selected = false;
      }
    }
  }
}

pub fn enemy_atlas_layout(
  mut query: Query<
    (&EnemyState, &Direction, &mut Sprite),
    (With<Enemy>, Or<(Changed<Direction>, Changed<EnemyState>)>),
  >,
  animation: Res<Animation<EnemyStates>>,
) {
  for (enemy_state, direction, mut sprite) in &mut query {
    sprite.texture_atlas.as_mut().unwrap().layout = animation
      .atlas_config
      .get(&enemy_state.value)
      .unwrap()
      .layouts
      .get(&direction.value)
      .unwrap()
      .clone();
  }
}

pub fn enemy_state(mut query: Query<(&mut EnemyState, &Movable), Changed<Movable>>) {
  for (mut enemy_state, movable) in &mut query {
    if movable.path.len() == 0 && enemy_state.value != EnemyStates::Idle {
      enemy_state.value = EnemyStates::Idle;
    }

    if movable.path.len() > 0 && enemy_state.value != EnemyStates::Walk {
      enemy_state.value = EnemyStates::Walk;
    }
  }
}
