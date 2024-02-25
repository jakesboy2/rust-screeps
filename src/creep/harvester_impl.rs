use screeps::Room;

use super::creep_lib::CreepStrategy;

#[derive(Default)]
pub struct Harvester {}

impl CreepStrategy for Harvester {
    fn get_energy(&self, _room: &Room) -> () {
      todo!()
    }
}