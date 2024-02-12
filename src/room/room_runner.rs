use screeps::Room;

use log::*;

use crate::{empire::empire_lib::get_owned_rooms, spawning::spawning_runner, visuals::room_visuals};

pub fn run_rooms() {
  let rooms = get_owned_rooms();
  for mut room in rooms {
    run_room(&mut room);
  }
}

fn run_room(room: &mut Room) {
  debug!("Running room {}", room.name().to_array_string());  
  spawning_runner::run_spawning(&room);    
  room_visuals::draw_room(&room);
}
