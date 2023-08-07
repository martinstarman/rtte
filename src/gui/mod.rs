///
pub mod character;

///
pub mod enemy;

///
pub mod object;

///
pub mod base;

///
pub mod mesh;

use crate::State;

use ggegui::egui::{self, Rect};

use self::{
  base::draw_base_gui, character::draw_character_gui, enemy::draw_enemy_gui,
  object::draw_object_gui, mesh::draw_mesh_gui,
};

pub fn update(state: &mut State) -> Option<Rect> {
  let gui_ctx = state.gui.ctx();

  //
  gui_ctx.set_visuals(egui::Visuals::light());

  let resp = egui::Window::new("rtte").show(&gui_ctx, |ui| {
    draw_base_gui(ui, state);
    draw_character_gui(ui, state);
    draw_enemy_gui(ui, state);
    draw_object_gui(ui, state);
    draw_mesh_gui(ui, state);
  });

  if resp.is_some() {
    Some(resp.unwrap().response.rect)
  } else {
    None
  }
}
