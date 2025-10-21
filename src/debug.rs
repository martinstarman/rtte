use bevy::{
  dev_tools::{fps_overlay::FpsOverlayConfig, picking_debug::DebugPickingMode},
  prelude::*,
};
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
  mut fps_config: ResMut<FpsOverlayConfig>,
  mut picking_mode: ResMut<DebugPickingMode>,
  mut commands: Commands,
) {
  if debug.enabled {
    debug.enabled = false;
    fps_config.enabled = false;
    fps_config.frame_time_graph_config.enabled = false;
    *picking_mode = DebugPickingMode::Disabled;
    commands.remove_resource::<NavMeshesDebug>();
  } else {
    debug.enabled = true;
    fps_config.enabled = true;
    fps_config.frame_time_graph_config.enabled = true;
    *picking_mode = DebugPickingMode::Normal;
    commands.insert_resource(NavMeshesDebug(Color::srgba(1., 1., 1., 0.25)));
  }
}
