use screeps::{
  game,
  Room,
  local::RoomName,
  js_collections::JsHashMap
};

use crate::room::room_runner;

pub fn run_empire() {
  let rooms: JsHashMap<RoomName, Room> = game::rooms();

  for mut room in rooms.values() {
    room_runner::run_room(&mut room);
  }
}
