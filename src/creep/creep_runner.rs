use screeps::{game, Creep, SharedCreepProperties};

use crate::memory::{memory_definitions::CreepRole, memory_lib::get_creep_memory};

use super::{creep_lib::CreepStrategy, harvester_impl::Harvester, worker_impl::Worker};

// keep
pub fn run_creeps() {
  let creeps = game::creeps().values();
  for creep in creeps {
    run_creep(&creep);
  }
}

pub fn run_creep(creep: &Creep) {
  log::debug!("Running creep {}", creep.name());
  let creep_memory = get_creep_memory(creep);
  let creep_strategy = get_creep_strategy(&creep_memory.role);

  // general idea --
  // get a task ->
  // perform that task
  // we will probably want to pass a strategy?
  
  // get strategy for role (accept T: Trait), pass to here
  // probably the same idea for the memory/name stuff
}

fn get_creep_strategy(creep_role: &CreepRole) -> Box<dyn CreepStrategy> {
  match creep_role {
    CreepRole::Harvester => Box::new(Harvester::default()),
    CreepRole::Worker => Box::new(Worker::default()),
  }
}