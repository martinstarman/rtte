use bevy::{asset::RenderAssetUsages, math::CompassOctant, mesh::PrimitiveTopology, prelude::*};
use std::collections::HashMap;

use crate::{
  animation::{Animation, AnimationAtlasConfig},
  cone_of_view::{ConeOfView, ConeOfViewShift, CONE_OF_VIEW_VERTICES},
  direction::Direction,
  movement::{Movement, PathItem, Speed::Slow},
  selection::Selection,
  utils::timer_from_fps,
  ysort::YSort,
};

pub const ENEMY_TILE_SIZE: Vec2 = Vec2::new(16., 32.);

pub struct EnemySpawn {
  position: Vec2,
  asset_path: String,
  path: Vec<PathItem>,
  direction: CompassOctant,
}

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
  Dead = 4,
}

impl Command for EnemySpawn {
  fn apply(self, world: &mut World) {
    let mut atlas_config = HashMap::new();
    let image = world.load_asset(self.asset_path);
    let tile_size = UVec2::new(16, 32);
    let directions = vec![
      CompassOctant::East,
      CompassOctant::NorthEast,
      CompassOctant::North,
      CompassOctant::NorthWest,
      CompassOctant::West,
      CompassOctant::SouthWest,
      CompassOctant::South,
      CompassOctant::SouthEast,
    ];

    // idle
    let mut layouts = HashMap::new();

    for (i, direction) in directions.iter().enumerate() {
      let offset = Some(UVec2::new(0, i as u32 * tile_size.y));
      let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 1, None, offset);
      let handle = world
        .resource_mut::<Assets<TextureAtlasLayout>>()
        .add(atlas);
      layouts.insert(direction.clone(), handle);
    }

    let config = AnimationAtlasConfig {
      fps: 10,
      frame_count: 4,
      layouts,
    };

    atlas_config.insert(EnemyStates::Idle, config);

    // walk
    let mut layouts = HashMap::new();

    for (i, direction) in directions.iter().enumerate() {
      let offset = Some(UVec2::new(0, (i as u32 * tile_size.y) + 256));
      let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 1, None, offset);
      let handle = world
        .resource_mut::<Assets<TextureAtlasLayout>>()
        .add(atlas);
      layouts.insert(direction.clone(), handle);
    }

    let config = AnimationAtlasConfig {
      fps: 10,
      frame_count: 4,
      layouts,
    };

    atlas_config.insert(EnemyStates::Walk, config);

    // run
    let mut layouts = HashMap::new();

    for (i, direction) in directions.iter().enumerate() {
      let offset = Some(UVec2::new(0, (i as u32 * tile_size.y) + 512));
      let atlas = TextureAtlasLayout::from_grid(tile_size, 4, 1, None, offset);
      let handle = world
        .resource_mut::<Assets<TextureAtlasLayout>>()
        .add(atlas);
      layouts.insert(direction.clone(), handle);
    }

    let config = AnimationAtlasConfig {
      fps: 10,
      frame_count: 4,
      layouts,
    };

    atlas_config.insert(EnemyStates::Run, config);

    // dead
    let mut layouts = HashMap::new();

    for (i, direction) in directions.iter().enumerate() {
      let offset = Some(UVec2::new(0, (i as u32 * tile_size.y) + 768));
      let atlas = TextureAtlasLayout::from_grid(tile_size, 1, 1, None, offset);
      let handle = world
        .resource_mut::<Assets<TextureAtlasLayout>>()
        .add(atlas);
      layouts.insert(direction.clone(), handle);
    }

    let config = AnimationAtlasConfig {
      fps: 1,
      frame_count: 1,
      layouts,
    };

    atlas_config.insert(EnemyStates::Dead, config);

    let default_fps = atlas_config.get(&EnemyStates::Idle).unwrap().fps;
    let default_layout = atlas_config
      .clone()
      .get(&EnemyStates::Idle)
      .unwrap()
      .layouts
      .get(&CompassOctant::South)
      .unwrap()
      .clone();

    world.insert_resource(Animation {
      frame_timer: timer_from_fps(default_fps),
      atlas_config,
    });

    let mesh = Mesh::new(
      PrimitiveTopology::TriangleList,
      RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
      Mesh::ATTRIBUTE_POSITION,
      [Vec3::ZERO; CONE_OF_VIEW_VERTICES].to_vec(),
    );
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let mesh_handle = meshes.add(mesh);
    let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
    let material_handle = materials.add(ColorMaterial {
      color: Color::srgba(0.0, 1.0, 0.0, 0.1),
      ..default()
    });

    world
      .spawn((
        Enemy,
        Movement {
          path: self.path.clone(),
          default_path: self.path.clone(),
        },
        EnemyState::default(),
        Direction::from(self.direction),
        Sprite {
          image,
          texture_atlas: Some(TextureAtlas::from(default_layout)),
          ..default()
        },
        Transform::from_translation(self.position.extend(0.)),
        YSort { height: 32 },
        ConeOfView {
          looking_at: Dir2::from(self.direction).normalize(),
          offset: 0,
          shift: ConeOfViewShift::Left,
          mesh_handle: mesh_handle.clone(),
        },
        Selection::default(),
        Pickable::default(),
      ))
      .with_children(|parent| {
        parent.spawn((
          Mesh2d(mesh_handle),
          MeshMaterial2d(material_handle),
          Visibility::Visible,
        ));
      })
      // .with_children(|parent| {
      //   parent.spawn((
      //     Transform::from_translation(Vec3::new(0., -12., 0.)),
      //     PrimitiveObstacle::Rectangle(Rectangle::new(16., 8.)),
      //   ));
      // })
      .observe(select_enemy::<Pointer<Press>>());
  }
}

pub fn enemy_init(mut commands: Commands) {
  commands.queue(EnemySpawn {
    position: Vec2::new(0., 100.),
    asset_path: String::from("enemy/export.png"),
    path: vec![
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
    ],
    direction: CompassOctant::North,
  });
  commands.queue(EnemySpawn {
    position: Vec2::new(-200., -200.),
    asset_path: String::from("enemy/export.png"),
    path: vec![],
    direction: CompassOctant::West,
  });
}

fn select_enemy<E: EntityEvent>() -> impl Fn(
  On<E>,
  Query<(Entity, &mut Selection, &EnemyState), With<Enemy>>,
  ResMut<ButtonInput<MouseButton>>,
) {
  move |event, mut query, mut mouse| {
    mouse.clear_just_pressed(MouseButton::Left);

    for (entity, mut selection, enemy_state) in &mut query {
      if enemy_state.value == EnemyStates::Dead {
        continue;
      }

      if entity == event.event_target() {
        selection.active = !selection.active;
      } else {
        selection.active = false;
      }
    }
  }
}

pub fn enemy_update_atlas_layout_on_direction_or_state_change(
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

pub fn enemy_update_state_on_movement_change(
  mut query: Query<(&mut EnemyState, &Movement), Changed<Movement>>,
) {
  for (mut enemy_state, movement) in &mut query {
    if enemy_state.value == EnemyStates::Dead {
      continue;
    }

    if movement.path.len() == 0 && enemy_state.value != EnemyStates::Idle {
      enemy_state.value = EnemyStates::Idle;
    }

    if movement.path.len() > 0 && enemy_state.value != EnemyStates::Walk {
      enemy_state.value = EnemyStates::Walk;
    }
  }
}

pub fn enemy_animation_tick(
  mut query: Query<(&EnemyState, &mut Sprite), With<Enemy>>,
  mut animation: ResMut<Animation<EnemyStates>>,
  time: Res<Time>,
) {
  for (enemy_state, mut sprite) in &mut query {
    animation.frame_timer.tick(time.delta());

    if animation.frame_timer.just_finished() {
      let atlas_config = animation.atlas_config.get(&enemy_state.value).unwrap();
      let frame_count: usize = if atlas_config.frame_count == 1 {
        1
      } else {
        atlas_config.frame_count as usize - 1
      };

      sprite.texture_atlas.as_mut().unwrap().index =
        (sprite.texture_atlas.as_mut().unwrap().index + 1) % frame_count;
      animation.frame_timer = timer_from_fps(atlas_config.fps);
    }
  }
}

pub fn enemy_reset_animation_on_state_change(
  mut query: Query<(&EnemyState, &mut Sprite), (With<Enemy>, Changed<EnemyState>)>,
  mut animation: ResMut<Animation<EnemyStates>>,
) {
  for (enemy_state, mut sprite) in &mut query {
    let atlas_config = animation.atlas_config.get(&enemy_state.value).unwrap();
    let frame_count: usize = if atlas_config.frame_count == 1 {
      1
    } else {
      atlas_config.frame_count as usize - 1
    };

    sprite.texture_atlas.as_mut().unwrap().index =
      (sprite.texture_atlas.as_mut().unwrap().index + 1) % frame_count;
    animation.frame_timer = timer_from_fps(atlas_config.fps);
  }
}
