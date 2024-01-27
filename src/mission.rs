use serde::Deserialize;
use std::{fs::File, io::Read};

use crate::entity::{
  enemy::EnemyEntity, image::ImageEntity, player::PlayerEntity, polygon::PolygonEntity,
};

#[derive(Deserialize)]
pub struct Mission {
  pub player: Vec<PlayerEntity>,
  pub enemy: Vec<EnemyEntity>,
  pub image: Vec<ImageEntity>,
  pub polygon: Vec<PolygonEntity>,
}

pub fn load(file_name: &str) -> Mission {
  let mut content = String::new();
  let mut file =
    File::open(file_name).expect(format!("Mission file {} not found.", file_name).as_str());

  file.read_to_string(&mut content).expect(format!("Unable to read file {}", file_name).as_str());

  toml::from_str(content.as_str()).unwrap()
}
