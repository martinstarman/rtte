use bevy_minibuffer::prelude::*;

pub fn console_init(mut minibuffer: Minibuffer) {
  minibuffer.message(">");
  minibuffer.set_visible(true);
}
