use crate::gamedef::material_def::MaterialDef;
use crate::gamedef::units::StructuralDurability;

pub enum MaterialState {
  /// For liquids defines level 1.0=full
  LiquidLevel(f32),
  /// For gases defines concentration vs air, 1.0 - 100% other gas, 0.0 - all air
  GasConcentration(f32),
  /// For solid materials defines how well the material holds if dug into
  /// When reaches 0, the material falls apart and produces a pile of that
  /// material on the ground (lootable).
  Solid(StructuralDurability),
}

/// The contents of the world cell.
pub struct WorldCell {
  /// Ambient temperature calculated from heat sources and neighbouring cells
  pub temperature: f32,
  /// Coverage by light sources/sky
  pub light_level: f32,

  /// Set to material, solid, liquid, also gases included, other than air.
  /// For air this is set to None
  pub material: Option<MaterialDef>,
  pub material_state: MaterialState,
}
