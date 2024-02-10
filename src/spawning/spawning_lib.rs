
use screeps::{
  constants::Part, game, Room
};

use crate::memory::memory_definitions::{CreepMemory, CreepRole};

// Use some form of strat pattern to grab these from a harvester implementation
pub fn get_creep_name(role: &CreepRole) -> String {
  return game::time().to_string() + &role.to_string();
}

// Use some form of strat pattern to grab these from a harvester implementation
pub fn get_creep_body(role: &CreepRole) -> &[Part] {
  match role {
    CreepRole::Harvester => &[Part::Work, Part::Carry, Part::Carry, Part::Move, Part::Move]
  }
}

// Use some form of strat pattern to grab these from a harvester implementation
pub fn get_creep_options(room: &Room, role: &CreepRole) -> CreepMemory {
  CreepMemory {
    role: role.clone(),
    home_room: room.name().to_string()
  }
}