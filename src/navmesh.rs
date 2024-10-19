use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResized};
use vleue_navigator::prelude::*;

pub fn navmesh_setup(mut commands: Commands) {
  commands.spawn(NavMeshBundle {
    settings: NavMeshSettings {
      fixed: Triangulation::from_outer_edges(&[
        Vec2::new(-400.0, -300.0),
        Vec2::new(400., -300.0),
        Vec2::new(400., 300.),
        Vec2::new(-400.0, 300.),
      ]),
      simplify: 0.05,
      ..default()
    },
    update_mode: NavMeshUpdateMode::Direct,
    transform: Transform::from_xyz(0., 0., -999.),
    ..NavMeshBundle::with_default_id()
  });

  // TODO: obstacles should be defined on tree
  let transform = Transform::from_translation(Vec3::new(-75., 60., 0.0));
  commands.spawn((
    PrimitiveObstacle::Rectangle(Rectangle::new(20., 20.)),
    SpatialBundle::from_transform(transform),
  ));
}

pub fn navmesh_draw(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  navmeshes: Res<Assets<NavMesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut current_mesh_entity: Local<Option<Entity>>,
  window_resized: EventReader<WindowResized>,
  navmesh: Query<(&Handle<NavMesh>, Ref<NavMeshStatus>)>,
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
      .spawn(MaterialMesh2dBundle {
        mesh: meshes.add(navmesh.to_mesh()).into(),
        material: materials.add(ColorMaterial::from(Color::srgba(0., 0., 1., 0.25))),
        ..default()
      })
      .with_children(|main_mesh| {
        main_mesh.spawn(MaterialMesh2dBundle {
          mesh: meshes.add(navmesh.to_wireframe_mesh()).into(),
          transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
          material: materials.add(ColorMaterial::from(Color::srgb(1., 0., 0.))),
          ..default()
        });
      })
      .id(),
  );
}

pub fn navmesh_obstacle_draw(mut gizmos: Gizmos, query: Query<(&PrimitiveObstacle, &Transform)>) {
  for (prim, transform) in &query {
    match prim {
      PrimitiveObstacle::Rectangle(prim) => {
        gizmos.primitive_2d(
          prim,
          transform.translation.xy(),
          transform.rotation.to_axis_angle().1,
          Color::srgb(1., 0., 0.),
        );
      }
      _ => panic!("Use reactangle"),
    }
  }
}