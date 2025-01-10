use bevy::{prelude::*, window::WindowResized};
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

pub fn navmesh_draw(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  navmeshes: Res<Assets<NavMesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut current_mesh_entity: Local<Option<Entity>>,
  window_resized: EventReader<WindowResized>,
  navmesh: Query<(&ManagedNavMesh, Ref<NavMeshStatus>)>,
) {
  let (navmesh_handle, status) = navmesh.single();

  if (!status.is_changed() || *status != NavMeshStatus::Built) && window_resized.is_empty() {
    return;
  }

  let Some(navmesh) = navmeshes.get(navmesh_handle) else {
    return;
  };

  if let Some(entity) = *current_mesh_entity {
    commands.entity(entity).despawn_recursive();
  }

  *current_mesh_entity = Some(
    commands
      .spawn((
        Mesh2d(meshes.add(navmesh.to_mesh())),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(0., 0., 1., 0.25)))),
      ))
      .with_children(|main_mesh| {
        main_mesh.spawn((
          Mesh2d(meshes.add(navmesh.to_wireframe_mesh()).into()),
          MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1., 0., 0.)))),
          Transform::from_xyz(0.0, 0.0, 0.1),
        ));
      })
      .id(),
  );
}
