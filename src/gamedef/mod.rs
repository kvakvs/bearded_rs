use crate::gamedef::chem_def::ChemDef;
use crate::gamedef::food_def::FoodDef;
use crate::gamedef::material_def::MaterialDef;
use crate::gamedef::material_def::Phase;

pub mod chem_def;
pub mod food_def;
pub mod material_def;
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
      materials: GameDef::define_materials(),
      foods: GameDef::define_foods(),
      chemicals: GameDef::define_chemicals(),
    }
  }

  fn define_foods() -> Vec<FoodDef> {
    vec![FoodDef {
      name: "apple",
      condition_degrade: 0.05,
      nutritional_value: 0.05,
      satiety: 0.25, // 4 apples and take a break
      joy: 0.05,
    }]
  }

  fn define_materials() -> Vec<MaterialDef> {
    vec![
      MaterialDef {
        name: "granite",
        density: 5.0,
        flammability: 0.0,
        t_conductivity: 0.0,
        phase: Phase::Solid,
      },
      MaterialDef {
        name: "air",
        density: 0.01,
        flammability: 0.0,
        t_conductivity: 0.01,
        phase: Phase::Gas,
      },
      MaterialDef {
        name: "water",
        density: 1.0,
        flammability: 0.0,
        t_conductivity: 0.5,
        phase: Phase::Liquid,
      },
    ]
  }

  fn define_chemicals() -> Vec<ChemDef> {
    vec![ChemDef {
      smell: None,
      smell_foulness: 0.0,
    }]
  }
}
