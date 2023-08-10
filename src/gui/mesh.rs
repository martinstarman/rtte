pub use crate::{geometry::vec2::Vec2, State};
use ggegui::egui::Ui;

pub fn draw_gui(ui: &mut Ui, state: &mut State) {
  let mut change = false;

  for triangle in state.mesh.triangles.iter_mut() {
    if triangle.is_selected {
      ui.separator();
      ui.label("Triangle");

      let mut is_path_block = triangle.is_path_block;
      let mut is_view_block = triangle.is_view_block;

      ui.horizontal(|ui| {
        ui.label("Blocking: ");
        ui.checkbox(&mut is_path_block, "path");
        ui.checkbox(&mut is_view_block, "view");
      });

      if triangle.is_path_block != is_path_block {
        triangle.is_path_block = is_path_block;
        change = true;
      }

      if triangle.is_view_block != is_view_block {
        triangle.is_view_block = is_view_block;
        change = true;
      }
    }
  }

  if change {
    state.mesh.update_pov_barriers();
  }
}
