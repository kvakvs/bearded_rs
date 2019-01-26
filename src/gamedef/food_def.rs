pub struct FoodDef {
  pub name: &'static str,
  /// Losing condition (spoiling) per turn
  pub condition_degrade: f32,
  /// How much energy body receives per round of meal
  pub nutritional_value: f32,
  /// How filling is the food, high satiety values fill the belly faster leaving
  /// the creature unable to eat more.
  pub satiety: f32,
  /// Increases joy when consumed (affects mood)
  pub joy: f32,
}

pub fn define_foods() -> Vec<FoodDef> {
  vec![FoodDef {
    name: "apple",
    condition_degrade: 0.05,
    nutritional_value: 0.05,
    satiety: 0.25, // 4 apples and take a break
    joy: 0.05,
  }]
}
