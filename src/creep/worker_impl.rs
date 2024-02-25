use screeps::Room;

use super::creep_lib::CreepStrategy;

#[derive(Default)]
pub struct Worker {}

impl CreepStrategy for Worker {
    fn get_energy(&self, _room: &Room) -> () {
      todo!()
    }
}