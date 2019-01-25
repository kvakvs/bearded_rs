/// Defines durability hitpoints or damage (dealt by tools and weapons).
/// Structural durability is the property of solid walls and floors.
/// The tools deal structural damage to the walls and floors to modify or dig them.
pub struct StructuralDurability(pub f32);

pub struct Health(pub f32);
pub struct Mana(pub f32);

// Physical Units
//

/// Kilograms
pub struct Weight(pub f32);
/// Cubic metres
pub struct Volume(pub f32);
/// Temperature, not Kelvin because reasons
pub struct Centigrade(pub f32);
