use macroquad::color::Color;

pub const DEBUG: bool = true;
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 600;
pub const PAN_SPEED: f32 = 5.;
pub const MIN_MOVEMENT_DISTANCE: f32 = 1.;
pub const RADIAN: f32 = std::f32::consts::PI / 180.;
pub const VIEW_DISTANCE: f32 = 150.; // TODO: fov_distance
pub const VIEW_INNER_ANGLE: f32 = 60. * RADIAN; // TODO: fov
pub const VIEW_SHIFT_MAX_ANGLE: f32 = 30. * RADIAN; // TODO: fov
pub const VIEW_COLOR: Color = Color::new(1., 0., 0., 0.5); // TODO: fov
