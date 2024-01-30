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
    room_visual.text(
      source.pos().x().u8().into(),
      source.pos().y().u8().into(),
      format!("{} / {}", source.energy(), source.energy_capacity()).into(),
      Some(TextStyle::default())
    );
  }
}