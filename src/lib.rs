use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap, HashSet},
};

use js_sys::{JsString, Object, Reflect};
use log::*;
use screeps::{
    game,
    local::ObjectId,
    objects::{Creep, Source, StructureController},
    prelude::*,
};
use wasm_bindgen::{prelude::*, JsCast};

mod logging;
mod visuals;
mod room;
mod empire;
mod job;

thread_local! {
    static CREEP_TARGETS: RefCell<HashMap<String, CreepTarget>> = RefCell::new(HashMap::new());
}

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

#[derive(Clone)]
enum CreepTarget {
    Upgrade(ObjectId<StructureController>),
    Harvest(ObjectId<Source>),
}

#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    INIT_LOGGING.call_once(|| {
        logging::setup_logging(logging::Debug);
    });
    // debug!("Loop starting! CPU: {}", game::cpu::get_used());

    empire::empire_runner::run_empire();

    // CREEP_TARGETS.with(|creep_targets_refcell| {
    //     let mut creep_targets = creep_targets_refcell.borrow_mut();
    //     debug!("running creeps");
    //     for creep in game::creeps().values() {
    //         run_creep(&creep, &mut creep_targets);
    //     }
    // });

    if game::time() % 1000 == 0 {
      clean_memory()
    }

    // info!("done! cpu: {}", game::cpu::get_used())
}

fn clean_memory() {
  info!("Running memory cleanup");
  let mut alive_creeps = HashSet::new();
  for creep_name in game::creeps().keys() {
    alive_creeps.insert(creep_name);
  }

  if let Ok(memory_creeps) = Reflect::get(&screeps::memory::ROOT, &JsString::from("creeps")) {
    let memory_creeps: Object = memory_creeps.unchecked_into();
    for creep_name_js in Object::keys(&memory_creeps).iter() {
      let creep_name = String::from(creep_name_js.dyn_ref::<JsString>().unwrap());

      if !alive_creeps.contains(&creep_name) {
        info!("deleting memory for dead creep {}", creep_name);
        let _ = Reflect::delete_property(&memory_creeps, &creep_name_js);
      }
    }
  }
}

fn run_creep(creep: &Creep, creep_targets: &mut HashMap<String, CreepTarget>) {
  if creep.spawning() {
      return;
  }
  let name = creep.name();
  debug!("Running creep {}", name);

  let target = creep_targets.entry(name);
  match target {
    Entry::Occupied(entry) => {
      // target found, ex handling
      // let creep_target = entry.get();
      // match on target possibilities and handle each
    }
    Entry::Vacant(entry) => {
      // no target, ex handling
      // find controller ->
      // entry.insert(CreepTarget::Upgrade(controller.id()))
    }
  }
}
