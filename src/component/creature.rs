use crate::component::inventory_item::InventoryItem;
use crate::gamedef::units::Health;
use crate::gamedef::units::Mana;

pub enum Effect {
  HealthGain(Health),
  ManaGain(Mana),
}


/// Auras are temporary or permanent buffs and debuffs present on a creature.
/// Auras can have duration. Auras can be granted by actions, spells, skills or
/// items either equipped or in possession.
pub struct Aura {
  pub name: &'static str,
  /// For auras with duration 0, the changes are applied immediately
  pub duration: usize,
  /// Effects are applied each turn for each active aura
  pub effects: Vec<Effect>,
}

pub enum AppendageType {
  Hand,
  Leg,
  Head,
  Paw,
  Tail,
}

pub struct Appendage {
  pub appendage_type: AppendageType,
  // TODO: list of damages, permanent and temporary
}

pub struct Creature {
  pub hp: Health,
  pub mp: Mana,
  pub auras: Vec<Aura>,
  /// Head, tail, hands, other limbs, mutations etc go here
  pub appendages: Vec<Appendage>,
  pub inventory: Vec<InventoryItem>,
}
