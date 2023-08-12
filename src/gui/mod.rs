pub mod base;
pub mod character;
pub mod enemy;
pub mod mesh;

use crate::{Mode, State};
use ggegui::egui::{self, Id, Pos2, Rect};

pub fn update(state: &mut State) -> Option<Rect> {
  let gui_ctx = state.gui.ctx();

  // dark visuals breaks gui on my laptot for some reason
  gui_ctx.set_visuals(egui::Visuals::light());

  let title = if state.mode == Mode::Runtime { "rtte (runtime mode)" } else { "rtte (edit mode)" };

  let resp = egui::Window::new(title)
    .id(Id::from("rtte"))
    .collapsible(false)
    .resizable(false)
    .title_bar(false)
    .fixed_pos(Pos2::new(560., 0.))
    .show(&gui_ctx, |ui| {
      // workaround for window size
      // @see https://github.com/emilk/egui/issues/498
      ui.set_height(590.);
      ui.set_width(200.);

      base::draw_gui(ui, state);
      character::draw_gui(ui, state);
      enemy::draw_gui(ui, state);
      mesh::draw_gui(ui, state);
    });

  if resp.is_some() {
    Some(resp.unwrap().response.rect)
  } else {
    None
  }
}
