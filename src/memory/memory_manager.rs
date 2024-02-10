use screeps::{Creep, SharedCreepProperties};
use serde::{Deserialize, Serialize};

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
pub struct CreepMemory {
  pub role: CreepRole,
  pub home_room: String
}

pub fn get_creep_memory(creep: &Creep) -> CreepMemory {
  let creep_memory = serde_wasm_bindgen::from_value(creep.memory());
  match creep_memory {
    Ok(mem) => {
      return mem;
    },
    Err(_) => {
      log::error!("Error unwrapping creep memory for creep {}", creep.name());
      creep_memory.unwrap()
    },
  }
}