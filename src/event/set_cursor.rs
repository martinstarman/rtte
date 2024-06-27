use bevy_ecs::event::Event;

use crate::resource::cursor::CursorType;

#[derive(Event)]
pub struct SetCursor {
  pub cursor_type: CursorType,
}
