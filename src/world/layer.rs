use crate::world::cell::WorldCell;

/// Represents a flat layer of map x*y, add more layers for depth
pub struct WorldLayer {
  pub cells: Vec<WorldCell>,
  // TODO: Registration of items and creatures in map cells or zones for performance
}

impl WorldLayer {
  pub fn new(size: [usize; 2]) -> Self {
    Self {
      cells: Vec::with_capacity(size[0] * size[1]),
    }
  }
}
