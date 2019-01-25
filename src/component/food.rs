use crate::gamedef::food_def::FoodDef;

/// Food component is created for objects which can be eaten.
pub struct Food {
  pub food_def: &'static FoodDef,
  /// Fresh = 1.0, rotten = 0.0
  pub condition: f32,
}
