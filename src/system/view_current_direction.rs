use bevy_ecs::system::Query;

use crate::{
  component::view::{Shift, ViewComponent},
  constants::RADIAN,
};

// TODO: fov_view_direction
pub fn run(mut query: Query<&mut ViewComponent>) {
  for mut view in &mut query {
    if view.shift == Shift::LEFT {
      view.current_direction += RADIAN;
    } else {
      view.current_direction -= RADIAN;
    }
  }
}
