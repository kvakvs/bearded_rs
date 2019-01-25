pub enum EquipSlot {
  MainHand,
  OffHand,
  Head,
  Body,
  Legs,
  Feet,
  Hands,
}

/// Component created for anything which can be equipped, magical items
/// can grant auras. Items which can be held in hands can also deal damage
/// to the selected target.
pub struct InventoryItem {
  pub equip_slot: EquipSlot,
}
