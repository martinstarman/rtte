use bevy::prelude::*;
use core::f32;

const RAD: f32 = f32::consts::PI / 180.;

#[derive(Component)]
pub struct LineOfSight {
  pub target: Vec2,
  pub polygon: Polygon<60>,
  pub shift: LineOfSightShift,
}

#[derive(Component, PartialEq, Eq)]
pub enum LineOfSightShift {
  Left = 0,
  Right = 1,
}

pub fn line_of_sight_setup(
  mut commands: Commands,
  // mut materials: ResMut<Assets<ColorMaterial>>,
  // mut meshes: ResMut<Assets<Mesh>>,
) {
  // let polygon = RegularPolygon::new(100., 5);
  // let mesh = meshes.add(polygon.mesh().build());
  // let material = materials.add(Color::WHITE);

  commands.spawn((
    LineOfSight {
      target: Vec2::Y.normalize(),
      polygon: Polygon {
        vertices: [Vec2::ZERO; 60],
      },
      shift: LineOfSightShift::Left,
    },
    // TODO: this must match target
    Transform::from_rotation(Quat::from_rotation_z(f32::consts::FRAC_PI_2)),
    // MaterialMesh2dBundle {
    //   mesh: mesh.into(),
    //   material: material.clone(),
    //   transform: Transform::from_xyz(0., 0., 0.),
    //   ..default()
    // },
  ));
}

// pub fn line_of_sight_update(mut query: Query<(&mut LineOfSight, &Transform)>) {
pub fn line_of_sight_update(mut query: Query<&mut LineOfSight>) {
  // for (mut line_of_sight, _transform) in &mut query {
  for mut line_of_sight in &mut query {
    // TODO
    line_of_sight.polygon.vertices[0] = Vec2::new(0., 0.);
    line_of_sight.polygon.vertices[2] = Vec2::new(50., 50.);
    line_of_sight.polygon.vertices[1] = Vec2::new(50., -50.);

    // for v in line_of_sight.polygon.vertices {
    //   // TODO: calculate current vertices
    // }
  }
}

pub fn line_of_sight_rotation(mut query: Query<(&LineOfSight, &mut Transform)>) {
  for (line_of_sight, mut transform) in &mut query {
    transform.rotate_z(if line_of_sight.shift == LineOfSightShift::Left {
      -RAD
    } else {
      RAD
    });
  }
}

pub fn line_of_sight_shift(mut query: Query<(&mut LineOfSight, &Transform)>) {
  for (mut line_of_sight, transform) in &mut query {
    let position = Vec2::new(transform.translation.x, transform.translation.y);
    let target_angle = ((line_of_sight.target - position).to_angle() + (2. * f32::consts::PI))
      % (2. * f32::consts::PI);

    let (axis, angle) = transform.rotation.to_axis_angle();
    let rotation_angle = ((angle * axis.z) + (2. * f32::consts::PI)) % (2. * f32::consts::PI);

    if f32::abs(target_angle - rotation_angle).to_degrees().floor() > 45. {
      line_of_sight.shift = if line_of_sight.shift == LineOfSightShift::Left {
        LineOfSightShift::Right
      } else {
        LineOfSightShift::Left
      }
    }
  }
}

pub fn line_of_sight_target(mut query: Query<&mut LineOfSight>) {
  for mut _line_of_sight in &mut query {
    // TODO: change target when enemy change path
  }
}

pub fn draw_line_of_sight(query: Query<(&LineOfSight, &Transform)>, mut gizmos: Gizmos) {
  for (line_of_sight, transform) in &query {
    let angle = transform.rotation.to_euler(EulerRot::ZYX).0;

    // TODO: do not use gizmos
    gizmos.primitive_2d(
      &line_of_sight.polygon,
      Vec2::new(100., -100.),
      angle,
      Color::WHITE,
    );
  }
}
