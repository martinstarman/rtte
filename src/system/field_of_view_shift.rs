use bevy_ecs::system::Query;

use crate::{
  component::field_of_view::{FieldOfViewComponent, Shift},
  constants::FIELD_OF_VIEW_SHIFT_MAX_ANGLE,
};

pub fn field_of_view_shift(mut query: Query<&mut FieldOfViewComponent>) {
  for mut field_of_view in &mut query {
    let angle = field_of_view.direction - field_of_view.movement_direction;

    if angle > FIELD_OF_VIEW_SHIFT_MAX_ANGLE {
      field_of_view.shift = Shift::RIGHT;
    }

    if angle < -FIELD_OF_VIEW_SHIFT_MAX_ANGLE {
      field_of_view.shift = Shift::LEFT;
    }
  }
}
