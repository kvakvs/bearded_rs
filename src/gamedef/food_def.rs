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
