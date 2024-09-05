use bevy::prelude::*;

#[derive(Component)]
pub struct LineOfSight {
  pub _target: Vec2, // TODO: use rotation
  pub polygon: Polygon<60>,
  pub shift: LineOfSightShift,
}

#[derive(Component, PartialEq, Eq)]
pub enum LineOfSightShift {
  Left = 0,
  // Right = 1,
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
      _target: Vec2::new(100., 100.),
      polygon: Polygon {
        vertices: [Vec2::ZERO; 60],
      },
      shift: LineOfSightShift::Left,
    },
    Transform::from_xyz(0., 0., 0.),
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
    line_of_sight.polygon.vertices[0] = Vec2::new(100., 100.);
    line_of_sight.polygon.vertices[1] = Vec2::new(200., 100.);
    line_of_sight.polygon.vertices[2] = Vec2::new(0., 0.);

    // for v in line_of_sight.polygon.vertices {
    //   // TODO: calculate current vertices
    // }
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

pub fn line_of_sight_shift(mut query: Query<(&LineOfSight, &mut Transform)>) {
  for (_line_of_sight, mut _transform) in &mut query {
    // let rotation = transform.translation.normalize();
    // let angle = (line_of_sight.target.normalize() - Vec2::new(rotation.x, rotation.y)).to_angle();
    // println!("{}", angle);
    // TODO: change between left and right shift
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
