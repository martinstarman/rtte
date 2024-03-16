use macroquad::color::Color;

pub const DEBUG: bool = true;
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 600;
pub const PAN_SPEED: f32 = 5.;
pub const MIN_MOVEMENT_DISTANCE: f32 = 1.;
pub const RADIAN: f32 = std::f32::consts::PI / 180.;
pub const FIELD_OF_VIEW_DISTANCE: f32 = 150.;
pub const FIELD_OF_VIEW_INNER_ANGLE: f32 = 60. * RADIAN;
pub const FIELD_OF_VIEW_SHIFT_MAX_ANGLE: f32 = 30. * RADIAN;
pub const FIELD_OF_VIEW_COLOR: Color = Color::new(1., 0., 0., 0.5);
pub const MOVEMENT_SPEED: f32 = 1.6;
