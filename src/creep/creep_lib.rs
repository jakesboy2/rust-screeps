use screeps::Part;

pub fn get_cost_for_body(parts: &[Part]) -> u32 {
  parts.iter().map(|p| p.cost()).sum()
}