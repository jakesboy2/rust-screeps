use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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
pub struct CreepMemory {
  pub role: CreepRole
}