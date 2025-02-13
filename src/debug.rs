use bevy::{dev_tools::fps_overlay::FpsOverlayConfig, prelude::*};
use vleue_navigator::NavMeshesDebug;

#[derive(Default, Resource)]
pub struct Debug {
  pub enabled: bool,
}

pub fn is_debug_enabled(debug: Res<Debug>) -> bool {
  debug.enabled
}

pub fn toggle_debug(
  mut debug: ResMut<Debug>,
  mut overlay: ResMut<FpsOverlayConfig>,
  mut commands: Commands,
) {
  if debug.enabled {
    debug.enabled = false;
    overlay.enabled = false;
    commands.remove_resource::<NavMeshesDebug>();
  } else {
    debug.enabled = true;
    overlay.enabled = true;
    commands.insert_resource(NavMeshesDebug(Color::srgba(1., 1., 1., 0.25)));
  }
}
