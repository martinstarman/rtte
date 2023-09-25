use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct ViewMark {
  /// Is view mark active.
  pub active: bool,

  /// X position.
  pub x: f32,

  /// Y position.
  pub y: f32,
}
