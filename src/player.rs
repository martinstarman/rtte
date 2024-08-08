use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn player_setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
  let texture_handle = asset_server.load("test.png");
  let texture_atlas = TextureAtlasLayout::from_grid(UVec2 { x: 256, y: 256 }, 4, 3, None, None);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  commands.spawn((
    Player,
    SpriteBundle {
      texture: texture_handle,
      ..default()
    },
    TextureAtlas::from(texture_atlas_handle),
  ));
}

pub fn player_animation(mut texture_atlas_q: Query<&mut TextureAtlas, With<Player>>) {
  for mut texture_atlas in &mut texture_atlas_q {
    texture_atlas.index = (texture_atlas.index + 1) % 8;
  }
}
