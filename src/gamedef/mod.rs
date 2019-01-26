use crate::gamedef::{
  chem_def::{define_chemicals, ChemDef},
  food_def::{define_foods, FoodDef},
  material_def::{define_materials, MaterialDef, Phase},
};

pub mod chem_def;
pub mod food_def;
pub mod material_def;
pub mod skill_def;
pub mod units;

/// Contains static global definitions which never change: physical materials,
/// items, auras/effects, creatures etc.
pub struct GameDef {
  pub materials: Vec<MaterialDef>,
  pub foods: Vec<FoodDef>,
  pub chemicals: Vec<ChemDef>,
}

impl GameDef {
  pub fn new() -> Self {
    GameDef {
      materials: define_materials(),
      foods: define_foods(),
      chemicals: define_chemicals(),
    }
  }
}
