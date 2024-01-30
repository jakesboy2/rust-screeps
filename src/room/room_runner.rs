use screeps::{
  game,
  Room
};

use log::*;

use crate::room::room_spawning;

use crate::visuals::room_visuals;

pub fn run_room(room: &mut Room) {

  debug!("Running room {}", room.name());  
  
  room_spawning::run_spawning(room);    

  room_visuals::draw_room(&room);

}
