use screeps::{
  game,
  Room
};

use crate::room::room_runner;

pub fn run_empire() {
  let rooms = game::rooms();

  for mut room in rooms.values() {
    room_runner::run_room(&mut room);
  }
}
