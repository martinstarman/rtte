use crate::{geometry::vec2::Vec2, State};
use ggegui::egui::{ComboBox, TextEdit, Ui};

pub fn draw_enemy_gui(ui: &mut Ui, state: &mut State) {
  for enemy in state.enemies.iter_mut() {
    if enemy.is_selected {
      ui.separator();
      ui.label("Enemy");

      let mut x = enemy.pos.x.to_string();
      let mut y = enemy.pos.y.to_string();

      ui.horizontal(|ui| {
        ui.label("x:");
        ui.add(TextEdit::singleline(&mut x).desired_width(50.));

        ui.label("y:");
        ui.add(TextEdit::singleline(&mut y).desired_width(50.));
      });

      let x = x.parse::<f32>();
      let y = y.parse::<f32>();

      let pos = Vec2::new(
        if x.is_ok() { x.unwrap() } else { enemy.pos.x },
        if y.is_ok() { y.unwrap() } else { enemy.pos.y },
      );

      if enemy.pos != pos {
        enemy.set_pos(pos);
      }

      ui.horizontal(|ui| {
        ui.label("Texture:");

        let mut res_path = enemy.res_path.clone();

        ComboBox::from_label("").selected_text(res_path.clone()).show_ui(ui, |ui| {
          for option in &state.resources {
            ui.selectable_value(&mut res_path, option.path.clone().into(), option.path.clone());
          }
        });

        let resource = state.resources.iter().find(|res| res.path == res_path);

        if resource.is_some() && enemy.res_path != resource.unwrap().path {
          enemy.set_resource(resource.unwrap().clone());
        }
      });
    }
  }
}
