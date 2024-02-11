use std::collections::HashSet;

use js_sys::{JsString, Object, Reflect};
use log::*;
use screeps::game;
use wasm_bindgen::{prelude::*, JsCast};

use crate::empire::empire_lib::setup_shared_memory;

pub mod logging;
pub mod visuals;
pub mod room;
pub mod empire;
pub mod creep;
pub mod memory;
pub mod spawning;

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    INIT_LOGGING.call_once(|| {
        logging::setup_logging(logging::Debug);
    });

    setup_shared_memory();
    empire::empire_runner::run_empire();

    if game::time() % 1000 == 0 {
      clean_creep_memory();
    }
}

fn clean_creep_memory() {
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
