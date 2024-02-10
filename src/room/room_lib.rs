use screeps::{game, Creep, Room};

use crate::memory::memory_manager::{get_creep_memory, CreepRole};

pub fn get_creeps_in_room(room: &Room) -> Vec<Creep> {
  let creeps_in_room = game::creeps().values().filter(|creep| {
    let creep_memory = get_creep_memory(creep);
    return creep_memory.home_room == room.name().to_string()
  });

  let return_val: Vec::<Creep> = creeps_in_room.collect();
  return_val
}

pub fn get_creep_count(room: &Room, role: &CreepRole) -> u32 {
  let mut sum = 0;
  let creeps_in_room = get_creeps_in_room(room);
  for creep in creeps_in_room {
    let creep_memory = get_creep_memory(&creep);
    if &creep_memory.role == role {
      sum += 1;
    }
  }
  sum
}