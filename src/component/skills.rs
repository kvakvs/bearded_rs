use crate::gamedef::skill_def::SkillDef;

pub struct Skill {
  /// Defines skill properties, name, etc.
  pub def: &'static SkillDef,
  /// Skill training level, defines rate of mistakes/botched actions
  pub level: f32,
  /// Multiplier for skill training, 0.0 - no training possible, 1.0 - regular
  /// training rate, 1.0+ passion, 2.0+ burning passion
  pub passion: f32,
}

/// Defines creature's available skills
pub struct Skills {
  pub skillset: Vec<Skill>,
}
