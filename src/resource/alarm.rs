use bevy_ecs::system::Resource;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};

#[derive(Resource)]
pub struct Alarm {
  active: bool,
  sound: Sound,
}

impl Alarm {
  pub async fn new() -> Self {
    let sound = load_sound("resources/siren.ogg").await.unwrap();

    Alarm {
      active: false,
      sound,
    }
  }

  pub fn set_active(&mut self) {
    if !self.active {
      self.active = true;

      let params = PlaySoundParams {
        looped: false,
        volume: 1.,
      };

      play_sound(&self.sound, params);
    }
  }
}
