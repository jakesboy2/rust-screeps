use screeps::{game, Creep, Room};
use std::{cell::RefCell, collections::HashMap};

use crate::{empire::empire_lib::get_owned_rooms, memory::{memory_definitions::CreepRole, memory_lib::get_creep_memory}};

thread_local! {
  static CREEP_COUNTS: RefCell<HashMap<String, HashMap<CreepRole, u32>>> = RefCell::new(HashMap::new())
}

pub fn setup_creep_counts() {
  CREEP_COUNTS.with(|creep_count_refcell| {
    let mut creep_counts = creep_count_refcell.borrow_mut();
    let rooms = get_owned_rooms();
    for room in rooms {
      creep_counts.insert(room.name().to_string(), get_creep_counts_as_hash(&room));
    }
  });
}

fn get_creep_counts_as_hash(room: &Room) -> HashMap<CreepRole, u32> {
  let creeps_in_room = get_creeps_in_room(room);
  let mut creep_count: HashMap<CreepRole, u32> = HashMap::new();

  for creep in creeps_in_room {
    let creep_memory = get_creep_memory(&creep);

    match creep_count.get(&creep_memory.role) {
      Some(count) => creep_count.insert(creep_memory.role, count + 1),
      None => creep_count.insert(creep_memory.role, 1)
    };
  }
  creep_count
}

pub fn get_creep_count(room: &Room, role: &CreepRole) -> u32 {
  CREEP_COUNTS.with(|creep_count_refcell| {
    let all_creep_counts = creep_count_refcell.borrow();

    let creep_counts = all_creep_counts.get(&room.name().to_string());
    if creep_counts.is_none() {
      log::error!("CREEP_COUNT not defined for room {}", &room.name().to_string());
      return u32::MAX;
    }
  
    let creep_count = creep_counts.unwrap().get(role);
    match creep_count {
      Some(value) => *value,
      None => 0
    }
  })
}

pub fn get_creeps_in_room(room: &Room) -> Vec<Creep> {
  let creeps_in_room = game::creeps().values().filter(|creep| {
    let creep_memory = get_creep_memory(creep);
    creep_memory.home_room == room.name().to_string()
  });

  let return_val: Vec::<Creep> = creeps_in_room.collect();
  return_val
}
