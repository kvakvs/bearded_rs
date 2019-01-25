use crate::game::GenIndex;

/// Placement component is created for objects which can be found in the world.
pub enum Placement {
  World(WorldPosition),
  Creature(GenIndex),
}

pub struct WorldPosition {
  pub xy: [f32; 2],
  pub depth: isize,
}
