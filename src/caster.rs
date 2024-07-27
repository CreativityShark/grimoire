use crate::spell;

#[derive(Debug)]
pub struct Caster {
    name: String,
    class: String,
    level: u32,
    spells: Vec<spell::Spell>,
}

impl Caster {
    pub fn build(name: &str, class: &str, level: u32) -> Self {
        Caster {
            name: String::from(name),
            class: String::from(class),
            level,
            spells: vec!(),
        }
    }
}

#[derive(Debug)]
pub struct SpellSlot {
    level: u32,
    used: bool,
}

impl SpellSlot {
    pub fn new(level: u32) -> Self {
        SpellSlot { level, used: false }
    }
}
