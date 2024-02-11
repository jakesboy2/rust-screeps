use screeps::{
  objects::*,
  prelude::*,
  Room,
  find::SOURCES
};

pub fn draw_room(room: &Room) {
  let room_visual = room.visual();
  let sources = room.find(SOURCES, Option::None);
  for source in sources {
    let x: f32 = source.pos().x().u8().into();
    let y: f32 = source.pos().y().u8().into();
    room_visual.text(
      x,
      y - 1.0,
      format!("{} / {}", source.energy(), source.energy_capacity()).into(),
      Some(TextStyle::default())
    );
  }
}