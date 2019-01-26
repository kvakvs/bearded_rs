/// Define phase of the matter
pub enum Phase {
  Gas,
  Solid,
  Liquid,
}

/// Physical properties of all materials in game
pub struct MaterialDef {
  pub name: &'static str,
  pub density: f32,
  /// 1.0 - supports burning indefinitely, > 1.0 explosive?
  pub flammability: f32,
  /// Thermal conductivity, cells/turn
  pub t_conductivity: f32,
  pub phase: Phase,
}

pub fn define_materials() -> Vec<MaterialDef> {
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
