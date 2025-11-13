use bevy::prelude::*;
use vleue_navigator::prelude::*;

use crate::map::Map;

pub fn navmesh_init(mut commands: Commands, map: Res<Map>) {
  let half_width = map.width / 2.0;
  let half_height = map.height / 2.0;

  commands.spawn((
    NavMeshSettings {
      fixed: Triangulation::from_outer_edges(&[
        Vec2::new(-half_width, -half_height),
        Vec2::new(half_width, -half_height),
        Vec2::new(half_width, half_height),
        Vec2::new(-half_width, half_height),
      ]),
      simplify: 0.05,
      agent_radius: 8.0,
      agent_radius_on_outer_edge: true,
      merge_steps: 5,
      ..default()
    },
    NavMeshUpdateMode::Direct,
    Transform::default(),
  ));
}
