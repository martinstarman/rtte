pub use crate::{
  geometry::{triangle, vec2::Vec2},
  State,
};
use ggegui::egui::{ComboBox, Ui};

pub fn draw_gui(ui: &mut Ui, state: &mut State) {
  let mut mesh_changed = false;

  for triangle in state.mesh.triangles.iter_mut() {
    if triangle.is_selected {
      ui.separator();
      ui.label("Triangle");

      let mut selected = triangle.kind;

      ComboBox::from_label("Kind").selected_text(format!("{:?}", selected)).show_ui(ui, |ui| {
        ui.selectable_value(&mut selected, triangle::Kind::GROUND, "Ground");
        ui.selectable_value(&mut selected, triangle::Kind::WATER, "Water");
        ui.selectable_value(&mut selected, triangle::Kind::SNOW, "Snow");
        ui.selectable_value(&mut selected, triangle::Kind::BLOCK, "Block");
        ui.selectable_value(&mut selected, triangle::Kind::TRANSPARENT, "Transparent");
      });

      if selected != triangle.kind {
        triangle.kind = selected;
        mesh_changed = true;
      }
    }
    // ui.label(".");
    // ui.label(".");
    // ui.label(".");
    // ui.label(".");
    // ui.label(".");
    // ui.label(".");
  }

  if mesh_changed {
    state.mesh.update_pov_barriers();
  }
}
