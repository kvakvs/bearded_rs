use crate::gamedef::material_def::MaterialDef;

/// Defines chemical properties of something. Also smell, taste?, acidity?, reactivity?
pub struct ChemDef {
  /// Things can smell of known physical materials
  pub smell: Option<&'static MaterialDef>,
  /// 1.0 strong foul smell, 0.0 neutral
  pub stench: f32,
}

pub fn define_chemicals() -> Vec<ChemDef> {
  vec![ChemDef {
    smell: None,
    stench: 0.0,
  }]
}
