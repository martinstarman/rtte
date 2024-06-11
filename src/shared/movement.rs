use std::str::FromStr;

use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, PartialEq)]
pub enum Movement {
  Idling,
  Walking,
  Running,
  Crawling,
}

impl FromStr for Movement {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "IDLING" => Ok(Movement::Idling),
      "WALKING" => Ok(Movement::Walking),
      "RUNNING" => Ok(Movement::Running),
      "CRAWLING" => Ok(Movement::Crawling),
      _ => Err(()),
    }
  }
}
