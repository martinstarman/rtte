use bevy_ecs::system::Query;

use crate::{
  component::view::{Shift, ViewComponent},
  constants::VIEW_SHIFT_MAX_ANGLE,
};

pub fn run(mut query: Query<&mut ViewComponent>) {
  for mut view in &mut query {
    let rad = view.current_direction - view.default_direction;

    if rad > VIEW_SHIFT_MAX_ANGLE {
      view.shift = Shift::RIGHT;
    }

    if rad < -VIEW_SHIFT_MAX_ANGLE {
      view.shift = Shift::LEFT;
    }
  }
}
