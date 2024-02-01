use js_sys::{JsString, Reflect, Object};
use log::{debug, warn};
use serde::{Serialize, Deserialize};
use screeps::{
  game,
  Room
};
use wasm_bindgen::{JsCast, JsValue};
use std::collections::HashMap;

use crate::room;
// temporary until we define these elsewhere
enum CreepRole {
  Harvester,
  Upgrader,
  Builder,
  Repairer,
  WallRepairer
}

struct GameMemory {
  rooms: HashMap<String, RoomMemory>,
  creeps: HashMap<String, CreepMemory>,
  flags: HashMap<String, FlagMemory>,
  spawns: HashMap<String, SpawnMemory>,
}

#[derive(Serialize, Deserialize)]
struct RoomMemory {
  //* If the field is required, you can declare the type directly
  //? room_state: String 

  //* If you need to rename the field to match JS, you can use the serde rename attribute
  //? #[serde(rename = "roomState")]

  //* If you want to skip serializing/deserializing a field, use the serde skip attribute
  //? #[serde(skip)]

  //* If the field is optional, you can use an Option with #[serde(default)] to set it to None if missing
  #[serde(default)]
  room_state: Option<String>
}


struct CreepMemory {
  role: CreepRole
}

struct FlagMemory {
  // TODO
}

struct SpawnMemory {
  // TODO
}


pub fn get_room_memory(room_name: &str) {

  debug!("Looking for room {} in memory", room_name);

  if let Ok(memory_rooms) = Reflect::get(&screeps::memory::ROOT, &JsString::from("rooms")) {
    if let Ok(room_memory) = Reflect::get(&memory_rooms, &JsString::from(room_name)) {
      debug!("Room {} found in memory", room_name);
      debug!("Memory: {:?}", room_memory);

      debug!("Pre-Conversion: Room State: {:?}", Reflect::get(&room_memory, &JsString::from("room_state")).unwrap());

      let room_memory: RoomMemory = serde_wasm_bindgen::from_value(room_memory).unwrap();

      if let Some(room_state) = room_memory.room_state {
        debug!("Room state: {}", room_state);
      } else {
        debug!("Room state not found");
      }

    } else {
      warn!("Memory.rooms[{}] not found", room_name);
    }
  } else {
    warn!("Memory.Rooms not found");
  }

}