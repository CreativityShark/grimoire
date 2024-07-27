#[derive(Debug)]
pub struct Spell {
    name: String,
    desc: String,
    school: String,
    cast_time: u32,
    duration: u32,
    prepared: bool,
}

impl Spell {
    pub fn cast(&mut self) -> Result<(), &'static str> {
        println!("abra cadabra!");

        Ok(())
    }

    pub fn new() -> Spell {
        Spell {
            name = String::from(""),
            desc = String::from(""),
            school = String::from(""),
            cast_time = 0,
            duration = 0,
            prepared = false,
        }
    }

    pub fn prepare(&mut self) {
        self.prepared = true;
    }

    pub fn unprepare(&mut self) {
        self.prepared = false;
    }
}
