use bevy::prelude::*;
use vleue_navigator::prelude::*;

pub fn navmesh_setup(mut commands: Commands) {
  commands.spawn((
    NavMeshSettings {
      fixed: Triangulation::from_outer_edges(&[
        Vec2::new(-400.0, -300.0),
        Vec2::new(400., -300.0),
        Vec2::new(400., 300.),
        Vec2::new(-400.0, 300.),
      ]),
      simplify: 0.05,
      ..default()
    },
    NavMeshUpdateMode::Direct,
    Transform::default(),
  ));
}
