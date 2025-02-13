use bevy_minibuffer::prelude::*;

pub fn console_setup(mut minibuffer: Minibuffer) {
  minibuffer.message(">");
  minibuffer.set_visible(true);
}
