use screeps::{Creep, Room};
use wasm_bindgen::UnwrapThrowExt;

use super::memory_definitions::{CreepMemory, RoomMemory};



pub fn get_creep_memory(creep: &Creep) -> CreepMemory {
  let creep_memory = serde_wasm_bindgen::from_value(creep.memory()).unwrap_throw();
  creep_memory
}

pub fn get_room_memory(room: &Room) -> RoomMemory {
  let room_memory = serde_wasm_bindgen::from_value(room.memory()).unwrap_throw();
  room_memory
}