use crate::gamedef::{
  material_def::MaterialDef,
  units::{Centigrade, Volume, Weight},
};

/// Component attached to everything which has physical properties and can have
/// physical transformations applied to it.
pub struct Physics {
  pub weight: Weight,
  pub volume: Volume,
  pub material: &'static MaterialDef,

  pub temperature: Centigrade,
  /// Objects can move freely in 2d, changing vertical coordinate only if dropped
  /// via some hole or carried.
  pub speed: [f32; 2],
  /// Objects can freely rotate in 2d
  pub rotation_speed: f32,
}
