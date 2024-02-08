use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum CreepRole {
  Harvester
}

#[derive(Serialize, Deserialize)]
pub struct CreepMemory {
  pub role: CreepRole
}