
use screeps::ResourceType;
use screeps::{
  constants::Part, game, Room, SpawnOptions, StructureSpawn
};

use crate::memory::memory_manager::{CreepMemory, CreepRole};
use crate::creep::creep_lib::get_cost_for_body;
use crate::room::room_lib::get_creep_count;

pub fn run_spawning(room: &Room) {
  log::debug!("Running spawns for room {}", room.name());
  for spawn in game::spawns().values() {
    run_spawn(&spawn, room);
  }
}

pub fn run_spawn(spawn: &StructureSpawn, room: &Room) {
  log::debug!("Running spawn for {}", spawn.name());
  if let Some(_) = spawn.spawning() {
    return ()
  }

  let role = get_creep_role_to_spawn(room);
  if role.is_none() {
    return ();
  }
  let role_to_spawn: &CreepRole = &role.unwrap();
  
  // Check if we have enough energy to spawn
  let body = get_creep_body(role_to_spawn);
  let energy_available = spawn.store().get(ResourceType::Energy).unwrap_or(0);
  let body_cost = get_cost_for_body(body);
  if body_cost > energy_available {
    return ()
  }
  
  let name = get_creep_name(role_to_spawn);
  let creep_memory = get_creep_options(role_to_spawn);
  let mut options = SpawnOptions::default();
  let mem = serde_wasm_bindgen::to_value(&creep_memory);

  if let Ok(spawn_options_memory) = mem {
    options = SpawnOptions::memory(options, spawn_options_memory);
    log::debug!("Spawning {}", name);

    let result = spawn.spawn_creep_with_options(body, name.as_str(), &options);
    if let Err(error_code) = result {
      log::debug!("Error spawning {}, error code: {:?}", name, error_code)
    }
  }
}

// Use some form of strat pattern to grab these from a harvester implementation
fn get_creep_name(role: &CreepRole) -> String {
  return game::time().to_string() + &role.to_string();
}

// Use some form of strat pattern to grab these from a harvester implementation
fn get_creep_body(role: &CreepRole) -> &[Part] {
  match role {
    CreepRole::Harvester => &[Part::Work, Part::Carry, Part::Carry, Part::Move, Part::Move]
  }
}

// Use some form of strat pattern to grab these from a harvester implementation
fn get_creep_options(role: &CreepRole) -> CreepMemory {
  CreepMemory {
    role: role.clone()
  }
}

fn get_creep_role_to_spawn(room: &Room) -> Option<CreepRole> {
  let harvester_count = get_creep_count(room, &CreepRole::Harvester);
  let max_harvesters = 2;

  if max_harvesters > harvester_count {
    return Some(CreepRole::Harvester);
  }

  None
}