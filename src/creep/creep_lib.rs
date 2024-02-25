use screeps::{Part, Room};

pub trait CreepStrategy {
  fn get_energy(&self, room: &Room) -> ();
}

pub fn get_cost_for_body(parts: &[Part]) -> u32 {
  parts.iter().map(|p| p.cost()).sum()
}