
use screeps::{
  game,
  Room,
  StructureSpawn
};

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
}