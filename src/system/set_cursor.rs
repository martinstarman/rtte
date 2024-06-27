use crate::{event::set_cursor::SetCursor, resource::cursor::Cursor};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn set_cursor(mut events: EventReader<SetCursor>, mut cursor: ResMut<Cursor>) {
  for event in events.read() {
    cursor.set_type(event.cursor_type);
  }
}
