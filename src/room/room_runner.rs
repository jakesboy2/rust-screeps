use screeps::{
  game,
  Room
};

use log::*;

use crate::room::room_spawning;

use crate::visuals::room_visuals;

use std::cell::RefCell;
use std::collections::HashMap;

use crate::memory::custom_memory::get_room_memory;

// thread_local! {
//   pub static ROOM_TARGETS : RefCell<HashMap<String, Room>> = RefCell::new(HashMap::new());
// }

pub fn run_room(room: &mut Room) {

  debug!("Running room {}", room.name().to_array_string() );  
  
  get_room_memory(&room.name().to_array_string());

  room_spawning::run_spawning(&room);    

  room_visuals::draw_room(&room);

}
