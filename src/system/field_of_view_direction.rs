use bevy_ecs::system::Query;

use crate::{
  component::field_of_view::{FieldOfViewComponent, Shift},
  constants::RADIAN,
};

pub fn field_of_view_direction(mut query: Query<&mut FieldOfViewComponent>) {
  for mut view in &mut query {
    if view.shift == Shift::LEFT {
      view.direction += RADIAN;
    } else {
      view.direction -= RADIAN;
    }
  }
}
