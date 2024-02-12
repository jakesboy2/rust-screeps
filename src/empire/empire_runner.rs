use crate::{creep::creep_runner::run_creeps, room::room_runner::run_rooms};

pub fn run_empire() {
  run_rooms();
  run_creeps();
}
