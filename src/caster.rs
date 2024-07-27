use crate::spell;

#[derive(Debug)]
pub struct Caster {
    pub name: String,
    pub class: String,
    pub level: u32,
    pub spells: Vec<spell::Spell>,
    pub slots: Vec<SpellSlot>,
}

impl Caster {
    pub fn new(name: &str, class: &str, level: u32) -> Self {
        Caster {
            name: String::from(name),
            class: String::from(class),
            level,
            spells: vec!(),
            slots: vec!(),
        }
    }

    pub fn rest(&mut self) {
        for mut slot in &mut self.slots {
            slot.restore_slot();
        }
    }
}

/// Representation of a spell slot used by `Caster`. Has basic functionality for using and
/// restoring spell slots.
///
/// # Examples
///
/// ```
/// use grimoire::caster::SpellSlot;
///
/// let mut slot = SpellSlot::new(2);
/// slot.use_slot();
///
/// assert!(slot.used);
/// ```

#[derive(Debug, PartialEq)]
pub struct SpellSlot {
    pub level: u32,
    pub used: bool,
}

impl SpellSlot {
    pub fn new(level: u32) -> Self {
        SpellSlot { level, used: false }
    }

    /// Sets `used` to true. If the spell slot has already been used, the method will return an
    /// error.
    ///
    /// ```should_panic
    /// // This code will panic!
    /// # use grimoire::caster::SpellSlot;
    /// # let mut slot = SpellSlot::new(1);
    /// slot.used = true;
    /// slot.use_slot().unwrap();
    /// ```
    pub fn use_slot(&mut self) -> Result<(), &'static str> {
        if self.used {
            return Err("Spell slot has already been used!");
        }
        self.used = true;

        Ok(())
    }

    /// Sets `used` to false. Unlike `use_slot()`, this method will not return an error if the slot
    /// is still unused.
    ///
    /// ```
    /// # use grimoire::caster::SpellSlot;
    /// # let mut slot = SpellSlot::new(1);
    /// slot.used = true;
    /// slot.restore_slot();
    ///
    /// assert!(!slot.used);
    /// ```
    pub fn restore_slot(&mut self) {
        self.used = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resting() {
        let mut caster = Caster::new("", "", 1);
        caster.slots.push(SpellSlot::new(1));
        caster.slots.push(SpellSlot::new(1));
        caster.slots[1].use_slot().unwrap();
        caster.rest();
        assert_eq!(caster.slots, vec!(SpellSlot { level: 1, used: false }, SpellSlot{ level: 1, used: false }));
    }
}
