use bevy_ecs::system::Query;

use crate::{
  component::field_of_view::{FieldOfViewComponent, Shift},
  constants::FIELD_OF_VIEW_SHIFT_MAX_ANGLE,
};

pub fn field_of_view_shift(mut query: Query<&mut FieldOfViewComponent>) {
  for mut view in &mut query {
    let rad = view.direction - view.movement_direction;

    if rad > FIELD_OF_VIEW_SHIFT_MAX_ANGLE {
      view.shift = Shift::RIGHT;
    }

    if rad < -FIELD_OF_VIEW_SHIFT_MAX_ANGLE {
      view.shift = Shift::LEFT;
    }
  }
}
