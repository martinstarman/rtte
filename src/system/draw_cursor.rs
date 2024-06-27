use bevy_ecs::system::Res;

use crate::resource::cursor::Cursor;

pub fn draw_cursor(cursor: Res<Cursor>) {
  cursor.draw();
}
