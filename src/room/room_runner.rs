use screeps::Room;

use log::*;

use crate::{spawning::spawning_runner, visuals::room_visuals};

pub fn run_room(room: &mut Room) {

  debug!("Running room {}", room.name().to_array_string() );  
  spawning_runner::run_spawning(&room);    
  room_visuals::draw_room(&room);
}
