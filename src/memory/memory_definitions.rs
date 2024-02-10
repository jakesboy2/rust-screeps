use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreepMemory {
  pub role: CreepRole,
  pub home_room: String
}

#[derive(Serialize, Deserialize)]
pub struct RoomMemory {
  pub room_state: RoomState
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum CreepRole {
  Harvester
}

impl ToString for CreepRole {
  fn to_string(&self) -> String {
    match self {
      Self::Harvester => "harvester".to_string()
    }
  }
}

#[derive(Serialize, Deserialize)]
pub enum RoomState {
  Intro
}
