use screeps::{game, Creep, SharedCreepProperties};

// keep
pub fn run_creeps() {
  let creeps = game::creeps().values();
  for creep in creeps {
    run_creep(&creep);
  }
}

pub fn run_creep(creep: &Creep) {
  log::debug!("Running creep {}", creep.name());
  // general idea --
  // get a task ->
  // perform that task
  // we will probably want to pass a strategy?
  
  // get strategy for role (accept T: Trait), pass to here
  // probably the same idea for the memory/name stuff
}