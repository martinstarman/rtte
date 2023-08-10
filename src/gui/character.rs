use crate::{geometry::vec2::Vec2, State};
use ggegui::egui::{TextEdit, Ui};

pub fn draw_gui(ui: &mut Ui, state: &mut State) {
  for character in state.characters.iter_mut() {
    if character.is_selected {
      ui.separator();
      ui.label("Character");

      let mut x = character.pos.x.to_string();
      let mut y = character.pos.y.to_string();

      ui.horizontal(|ui| {
        ui.label("x:");
        ui.add(TextEdit::singleline(&mut x).desired_width(50.));

        ui.label("y:");
        ui.add(TextEdit::singleline(&mut y).desired_width(50.));
      });

      let x = x.parse::<f32>();
      let y = y.parse::<f32>();

      let pos = Vec2::new(
        if x.is_ok() { x.unwrap() } else { character.pos.x },
        if y.is_ok() { y.unwrap() } else { character.pos.y },
      );

      if character.pos != pos {
        character.set_pos(pos);
      }
    }
  }
}
