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
