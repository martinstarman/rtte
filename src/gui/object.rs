use crate::{geometry::vec2::Vec2, State};
use ggegui::egui::{ComboBox, TextEdit, Ui};

pub fn draw_object_gui(ui: &mut Ui, state: &mut State) {
  for object in state.objects.iter_mut() {
    if object.is_selected {
      ui.separator();
      ui.label("Object");

      let mut x = object.pos.x.to_string();
      let mut y = object.pos.y.to_string();

      ui.horizontal(|ui| {
        ui.label("x:");
        ui.add(TextEdit::singleline(&mut x).desired_width(50.));

        ui.label("y:");
        ui.add(TextEdit::singleline(&mut y).desired_width(50.));
      });

      let x = x.parse::<f32>();
      let y = y.parse::<f32>();

      let pos = Vec2::new(
        if x.is_ok() { x.unwrap() } else { object.pos.x },
        if y.is_ok() { y.unwrap() } else { object.pos.y },
      );

      if object.pos != pos {
        object.set_pos(pos);
      }

      let mut w = object.size.x.to_string();
      let mut h = object.size.y.to_string();

      ui.horizontal(|ui| {
        ui.label("w:");
        ui.add(TextEdit::singleline(&mut w).desired_width(50.));
        ui.label("h:");
        ui.add(TextEdit::singleline(&mut h).desired_width(50.));
      });

      let w = w.parse::<f32>();
      let h = h.parse::<f32>();

      let size = Vec2::new(
        if w.is_ok() { w.unwrap() } else { object.size.x },
        if h.is_ok() { h.unwrap() } else { object.size.y },
      );

      if object.size != size {
        object.set_size(size);
      }

      ui.horizontal(|ui| {
        ui.label("Texture:");

        let mut res_path = object.res_path.clone();

        ComboBox::from_label("").selected_text(res_path.clone()).show_ui(ui, |ui| {
          for option in &state.resources {
            ui.selectable_value(&mut res_path, option.path.clone().into(), option.path.clone());
          }
        });

        let resource = state.resources.iter().find(|res| res.path == res_path);

        if resource.is_some() && object.res_path != resource.unwrap().path {
          object.set_resource(resource.unwrap().clone());
        }
      });
    }
  }
}
