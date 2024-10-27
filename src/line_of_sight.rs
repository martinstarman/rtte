use bevy::prelude::*;
use core::f32;

const DISTANCE: i32 = 150;
const INNER_ANGLE: i32 = 46;
const VERTICES: usize = INNER_ANGLE as usize + 1;

#[derive(Component)]
pub struct LineOfSight {
  /// where is line of sight looking
  pub looking_at: Vec2,

  /// offset from looking_at in degrees in range <-INNER_ANGLE/2;+INNER_ANGLE/2>
  pub offset: i32,

  /// current line of sight shift
  pub shift: LineOfSightShift,

  ///
  pub polygon: Polygon<VERTICES>,
}

#[derive(Component, PartialEq, Eq)]
pub enum LineOfSightShift {
  Left = 0,
  Right = 1,
}

pub fn line_of_sight_setup(mut commands: Commands) {
  commands.spawn((
    LineOfSight {
      looking_at: Vec2::new(-100., 40.).normalize() * DISTANCE as f32,
      offset: 0,
      shift: LineOfSightShift::Left,
      polygon: Polygon::new([Vec2::ZERO; VERTICES]),
    },
    Transform {
      rotation: Quat::default(),
      translation: Vec3::default(),
      ..Default::default()
    },
  ));
}

pub fn line_of_sight_update(mut query: Query<(&mut LineOfSight, &Transform)>) {
  for (mut line_of_sight, transform) in &mut query {
    let position = Vec2::new(transform.translation.x, transform.translation.y);
    let looking_at = line_of_sight.looking_at;

    let mut center_transform = Transform::from_xyz(looking_at.x, looking_at.y, 0.);

    let start_angle = line_of_sight.offset - INNER_ANGLE / 2;

    center_transform.rotate_around(
      Vec3::new(position.x, position.y, 0.),
      Quat::from_axis_angle(Vec3::Z, (start_angle as f32).to_radians()),
    );

    let mut points = [Vec2::ZERO; VERTICES];

    points[0] = position;

    for i in 0..INNER_ANGLE {
      center_transform.rotate_around(
        Vec3::new(position.x, position.y, 0.),
        Quat::from_axis_angle(Vec3::Z, (1 as f32).to_radians()),
      );

      points[i as usize + 1] = Vec2::new(
        center_transform.translation.x,
        center_transform.translation.y,
      );
    }

    line_of_sight.polygon = Polygon::new(points);
  }
}

pub fn line_of_sight_shift(mut query: Query<&mut LineOfSight>) {
  for mut line_of_sight in &mut query {
    line_of_sight.offset += if line_of_sight.shift == LineOfSightShift::Left {
      1
    } else {
      -1
    };

    if line_of_sight.offset >= INNER_ANGLE / 2 {
      line_of_sight.shift = LineOfSightShift::Right;
    }

    if line_of_sight.offset <= -INNER_ANGLE / 2 {
      line_of_sight.shift = LineOfSightShift::Left;
    }
  }
}

pub fn line_of_sight_target(mut _query: Query<(&LineOfSight, &Transform)>) {
  // TODO: change target when enemy change path
}

pub fn line_of_sight_draw(query: Query<(&LineOfSight, &Transform)>, mut gizmos: Gizmos) {
  for (line_of_sight, transform) in &query {
    let rect = Rectangle::new(10., 10.);

    let position = Vec2::new(transform.translation.x, transform.translation.y);
    let looking_at = line_of_sight.looking_at;

    gizmos.primitive_2d(&rect, position, 0., Color::WHITE);
    gizmos.primitive_2d(&rect, looking_at, 0., Color::WHITE);

    let mut center_transform = Transform::from_xyz(looking_at.x, looking_at.y, 0.);

    center_transform.rotate_around(
      Vec3::new(position.x, position.y, 0.),
      Quat::from_axis_angle(Vec3::Z, (line_of_sight.offset as f32).to_radians()),
    );

    gizmos.primitive_2d(
      &rect,
      Vec2::new(
        center_transform.translation.x,
        center_transform.translation.y,
      ),
      0.,
      Color::WHITE,
    );

    gizmos.primitive_2d(&line_of_sight.polygon, position, 0., Color::WHITE);
  }
}