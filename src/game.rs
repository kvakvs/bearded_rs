use crate::{
  component::{
    chemistry::Chemistry, creature::Creature, food::Food, inventory_item::InventoryItem,
    personality::Personality, physics::Physics, placement::Placement, skills::Skills,
  },
  world::layer::WorldLayer,
};

/// Generational index, allows storing game objects in the cells of the
/// game component arrays, while avoiding accidental use of a new object by
/// references to something which was here before.
pub struct GenIndex {
  pub index: usize,
  pub gen: u64,
}

pub struct Game {
  pub creature: Vec<Creature>,
  pub personality: Vec<Personality>,
  pub skills: Vec<Skills>,
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
      personality: vec![],
      skills: vec![],
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
