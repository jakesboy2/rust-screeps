
use screeps::{
  game,
  Room,
  StructureSpawn
};

pub fn run_spawning(room: &mut Room) {
  log::debug!("Running spawns for room {}", room.name());
  for spawn in game::spawns().values() {
    run_spawn(&spawn);
  }
}

pub fn run_spawn(spawn: &StructureSpawn) {
  log::debug!("Running spawn for {}", spawn.name());
}