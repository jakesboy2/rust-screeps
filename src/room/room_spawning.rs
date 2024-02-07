
use screeps::{
  game,
  Room,
  StructureSpawn,
  constants::Part,
  SpawnOptions
};
use wasm_bindgen::JsValue;
use js_sys::Object;

use crate::memory::memory_manager::{CreepMemory, CreepRole};

pub fn run_spawning(room: &Room) {
  log::debug!("Running spawns for room {}", room.name());
  for spawn in game::spawns().values() {
    run_spawn(&spawn);
  }
}

pub fn run_spawn(spawn: &StructureSpawn) {
  if let Some(_) = spawn.spawning() {
    return ()
  }
  log::debug!("Running spawn for {}", spawn.name());

  let max_harvesters = 2;
  let harvester_count = 0;

  if harvester_count > max_harvesters {
    let role = &CreepRole::Harvester;
    let name = get_creep_name(role);
    let body = get_creep_body(role);
    let creep_memory = get_creep_options(role);
    let mut options = SpawnOptions::default();
    // need to figure out mem here
    // probably move this all into get_creep_options to hide it away
    options = SpawnOptions::memory(options, mem);

    spawn.spawn_creep_with_options(body, name, opts);
  }
}

// Use some form of strat pattern to grab these from a harvester implementation
fn get_creep_name(role: &CreepRole) -> &str {
  return "";
}

// Use some form of strat pattern to grab these from a harvester implementation
fn get_creep_body(role: &CreepRole) -> &[Part] {
  &[Part::Work, Part::Carry, Part::Carry, Part::Move, Part::Move]
}

// Use some form of strat pattern to grab these from a harvester implementation
fn get_creep_options(role: &CreepRole) -> CreepMemory {
  CreepMemory {
    role: *role.clone()
  }
}