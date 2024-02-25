use bevy_ecs::system::Query;

use crate::{
  component::field_of_view::{FieldOfViewComponent, Shift},
  constants::RADIAN,
};

pub fn field_of_view_direction(mut query: Query<&mut FieldOfViewComponent>) {
  for mut field_of_view in &mut query {
    field_of_view.direction += if field_of_view.shift == Shift::LEFT { RADIAN } else { -RADIAN };
  }
}
