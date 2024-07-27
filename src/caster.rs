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

    pub fn use_slot(&mut self) -> Result<(), &'static str> {
        if self.used {
            return Err("Spell slot has already been used!");
        }
        self.used = true;

        Ok(())
    }

    pub fn restore_slot(&mut self) {
        self.used = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_unused_slot() {
        let mut slot = SpellSlot::new(1);
        assert_eq!(slot.use_slot(), Ok(()));
    }

    #[test]
    #[should_panic]
    fn use_used_slot() {
        let mut slot = SpellSlot::new(1);
        slot.used = true;
        slot.use_slot().unwrap();
    }

    #[test]
    fn restore_slot() {
        let mut slot = SpellSlot::new(1);
        slot.used = true;
        slot.restore_slot();
        assert!(!slot.used);
    }
}
