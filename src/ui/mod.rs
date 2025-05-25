use bevy::prelude::*;

pub mod actions;
pub mod players;

pub const UI_BG_COLOR: Color = Color::srgba(0.65, 0.65, 0.65, 0.1);
pub const UI_ITEM_BG_COLOR_BASE: Color = Color::srgb(0., 0., 0.);
pub const UI_ITEM_BG_COLOR_SELECTED: Color = Color::srgb(1., 0., 0.);
