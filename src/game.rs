use crate::component::creature::Creature;
use crate::component::food::Food;
use crate::component::inventory_item::InventoryItem;
use crate::component::physics::Physics;
use crate::component::placement::Placement;
use crate::world::layer::WorldLayer;
use crate::component::chemistry::Chemistry;

/// Generational index, allows storing game objects in the cells of the
/// game component arrays, while avoiding accidental use of a new object by
/// references to something which was here before.
pub struct GenIndex {
  pub index: usize,
  pub gen: u64
}

pub struct Game {
  pub creature: Vec<Creature>,
  pub placement: Vec<Placement>,
  pub physics: Vec<Physics>,
  pub chemistry: Vec<Chemistry>,
  pub food: Vec<Food>,
  pub inventory_item: Vec<InventoryItem>,

  pub world: Vec<Box<WorldLayer>>,
}

impl Game {
  pub fn new(sz: [usize; 2]) -> Self {
    Self {
      creature: Vec::new(),
      placement: Vec::new(),
      physics: Vec::new(),
      chemistry: Vec::new(),
      food: Vec::new(),
      inventory_item: Vec::new(),
      // Start with 2 layers, ground and walls
      world: vec![Box::new(WorldLayer::new(sz)), Box::new(WorldLayer::new(sz))],
    }
  }

  pub fn new_game(&mut self) {
    // TODO: generate for settlers their backstories
    // spawn settlers

  }
}
