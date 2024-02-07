use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CreepRole {
  Harvester
}

#[derive(Serialize, Deserialize)]
pub struct CreepMemory {
  pub role: CreepRole 
}