use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  // let polygon: Polygon<3> = Polygon {
  //   vertices: [
  //     Vec2::new(100., 100.),
  //     Vec2::new(200., 100.),
  //     Vec2::new(200., 200.),
  //   ],
  // };
  let polygon = RegularPolygon::new(100., 5);
  let mesh = meshes.add(polygon.mesh().build());
  let material = materials.add(Color::WHITE);

  commands.spawn((
    LineOfSight {
      target: Vec2::new(100., 100.),
      polygon: Polygon {
        vertices: [Vec2::ZERO; 60],
      },
      shift: LineOfSightShift::Left,
    },
    MaterialMesh2dBundle {
      mesh: mesh.into(),
      material: material.clone(),
      transform: Transform::from_xyz(0., 0., 0.),
      ..default()
    },
  ));
}

pub fn line_of_sight_update(mut query: Query<(&LineOfSight, &Transform)>) {
  for (mut line_of_sight, transform) in &mut query {
    let x = transform.translation.x;
    let y = transform.translation.y;

    for v in line_of_sight.polygon.vertices {
      // TODO: calculate current vertices
    }
  }
}

pub fn line_of_sight_rotation(mut query: Query<(&LineOfSight, &mut Transform)>, time: Res<Time>) {
  for (line_of_sight, mut transform) in &mut query {
    let rotation = Quat::from_mat3(&Mat3::from_angle(time.elapsed_seconds()));

    transform.rotation = if line_of_sight.shift == LineOfSightShift::Left {
      -rotation
    } else {
      rotation
    };
  }
}

pub fn line_of_sight_shift(mut query: Query<&mut LineOfSight>) {
  for mut line_of_sight in &mut query {
    // TODO: change between left and right shift
  }
}

pub fn line_of_sight_target(mut query: Query<&mut LineOfSight>) {
  for mut line_of_sight in &mut query {
    // TODO: change target when enemy change path
  }
}
