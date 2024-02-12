use screeps::{game, OwnedStructureProperties, Room};

use crate::room::room_lib::setup_creep_counts;

pub fn setup_shared_memory() {
  setup_creep_counts();
}

pub fn get_owned_rooms() -> Vec<Room> {
  let rooms: Vec<Room> = game::rooms().values().filter(|room| {
    if let Some(controller) = room.controller() {
      return controller.my() == true;
    }
    return false
  }).collect();
  
  rooms
}