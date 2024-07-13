pub fn run() {
    println!("epic!");
}

#[derive(Debug)]
pub enum CommandKind {
    TestSay(String),
    TestAdd(Vec<i32>),
}

#[derive(Debug)]
pub struct Command {
    pub kind: CommandKind,
}

impl Command {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Command, &'static str> {
        // First item is always name of program, use next() to get rid of it
        args.next();

        let kind = match args.next() {
            Some(cmd) => match Self::get_command_type(&cmd, args) {
                Ok(kind) => kind,
                Err(e) => return Err(e),
            },
            None => return Err("Missing a command!"),
        };

        Ok(Command { kind })
    }

    fn get_command_type(cmd: &str, mut args: impl Iterator<Item = String>) -> Result<CommandKind, &'static str> {
        match cmd {
            "add" => {
                let nums: Vec<i32> = args 
                    .filter_map(|n| {
                        match n.parse::<i32>() {
                            Ok(x) => Some(x),
                            _ => None,
                        }
                    }).collect();

                if nums.len() <= 0 { return Err("Missing number(s) to add!"); }

                Ok(CommandKind::TestAdd(nums))
            },
            "say" => {
                let message = match args.next() {
                    Some(s) => String::from(s),
                    None => return Err("Missing a message!"),
                };

                Ok(CommandKind::TestSay(message))
            },
            _ => return Err("Invalid command!"),
        }
    }
}
