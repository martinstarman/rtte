pub use crate::{geometry::vec2::Vec2, State};
use ggegui::egui::Ui;

pub fn draw_mesh_gui(ui: &mut Ui, state: &mut State) {
  let mut some_triangle_change = false;

    for triangle in state.mesh.triangles.iter_mut() {
      if triangle.is_selected {
        ui.separator();
        ui.label("triangle");

        let mut is_blocking_path = triangle.is_path_block;
        ui.checkbox(&mut is_blocking_path, "blocking path");

        if triangle.is_path_block != is_blocking_path {
          triangle.is_path_block = is_blocking_path;
          some_triangle_change = true;
        }

        let mut is_blocking_view = triangle.is_view_block;
        ui.checkbox(&mut is_blocking_view, "blocking view");

        if triangle.is_view_block != is_blocking_view {
          triangle.is_view_block = is_blocking_view;
          some_triangle_change = true;
        }
      }
    }

    if some_triangle_change {
      state.mesh.update_pov_barriers();
    }
}
