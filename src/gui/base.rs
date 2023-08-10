use crate::State;
use ggegui::egui::{Slider, Ui};

const W: f32 = 70.;
const H: f32 = 20.;

pub fn draw_gui(ui: &mut Ui, state: &mut State) {
  // 1st row
  ui.horizontal(|ui| {
    let btn_save = ui.add_sized([W, H], ggegui::egui::Button::new("Save"));
    let btn_load = ui.add_sized([W, H], ggegui::egui::Button::new("Load"));
    let btn_mode = ui.add_sized([W, H], ggegui::egui::Button::new("Mode"));

    if btn_save.clicked() {
      state.save();
    }

    if btn_load.clicked() {
      state.load();
    }

    if btn_mode.clicked() {
      state.toggle_mode();
    }
  });

  // 2nd row
  ui.horizontal(|ui| {
    let btn_character = ui.add_sized([W, H], ggegui::egui::Button::new("Character"));
    let btn_enemy = ui.add_sized([W, H], ggegui::egui::Button::new("Enemy"));

    if btn_character.clicked() {
      state.add_character();
    }

    if btn_enemy.clicked() {
      state.add_enemy();
    }
  });

  // 3rd row
  ui.horizontal(|ui| {
    let mut scale = state.scale.x;
    let slider = Slider::new(&mut scale, 0.1..=2.0).text("Scale");

    ui.add(slider);

    state.scale.x = scale;
    state.scale.y = scale;
  });
}
