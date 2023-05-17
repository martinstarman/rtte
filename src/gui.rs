use crate::geometry::vec2::Vec2;
use crate::State;

use ggegui::egui::{self, ComboBox, Rect, Slider, TextEdit};

pub fn update(state: &mut State) -> Option<Rect> {
  let gui_ctx = state.gui.ctx();
  gui_ctx.set_visuals(egui::Visuals::light());

  let resp = egui::Window::new("rtte").show(&gui_ctx, |ui| {
    //
    ui.horizontal(|ui| {
      if ui.button("serialize").clicked() {
        state.save();
      }

      if ui.button("deserialize").clicked() {
        state.load();
      }
    });

    //
    ui.horizontal(|ui| {
      if ui.button("enemy").clicked() {
        state.add_enemy();
      }

      if ui.button("character").clicked() {
        state.add_character();
      }
    });

    //
    let mode = match state.mode {
      crate::Mode::Edit => "runtime mode",
      crate::Mode::Runtime => "edit mode",
    };
    if ui.button(mode).clicked() {
      state.toggle_mode();
    }

    //
    let mut scale = state.scale.x;
    ui.add(Slider::new(&mut scale, 0.1..=2.0).text("scale"));
    state.scale.x = scale;
    state.scale.y = scale;

    //
    let mut some_triangle_change = false;

    for triangle in state.mesh.triangles.iter_mut() {
      if triangle.is_selected {
        ui.separator();
        ui.label("triangle");

        let mut is_blocking_path = triangle.is_blocking_path;
        ui.checkbox(&mut is_blocking_path, "blocking path");

        if triangle.is_blocking_path != is_blocking_path {
          triangle.is_blocking_path = is_blocking_path;
          some_triangle_change = true;
        }

        let mut is_blocking_view = triangle.is_blocking_view;
        ui.checkbox(&mut is_blocking_view, "blocking view");

        if triangle.is_blocking_view != is_blocking_view {
          triangle.is_blocking_view = is_blocking_view;
          some_triangle_change = true;
        }
      }
    }

    if some_triangle_change {
      state.mesh.update_pov_barriers();
    }

    //
    for character in state.characters.iter_mut() {
      if character.is_selected {
        ui.separator();
        ui.label("character");

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
          character.pos(pos);
        }
      }
    }

    //
    for enemy in state.enemies.iter_mut() {
      if enemy.is_selected {
        ui.separator();
        ui.label("enemy");

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
          enemy.pos(pos);
        }

        // texture
        ui.horizontal(|ui| {
          ui.label("texture:");

          let mut path = enemy.texture_path.clone();

          ComboBox::from_label("").selected_text(path.clone()).show_ui(ui, |ui| {
            for option in &state.resources {
              ui.selectable_value(&mut path, option.into(), option);
            }
          });

          if path != enemy.texture_path {
            enemy.set_texture_path(path);
          }
        });
      }
    }
  });

  if resp.is_some() {
    Some(resp.unwrap().response.rect)
  } else {
    None
  }
}
