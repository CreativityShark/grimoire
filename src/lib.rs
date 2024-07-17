use std::error::Error;

pub fn run(command: Box<dyn Command>) -> Result<(), Box<dyn Error>> {
    command.run()?;

    Ok(())
}

pub fn build_command(mut args: impl Iterator<Item = String>) -> Result<Box<dyn Command>, &'static str> {
    // First argument is program itself, skip with next()
    args.next();

    let cmd = match args.next() {
        Some(cmd) => cmd,
        None => return Err("Missing command!"),
    };

    if &cmd[..] == "add" {
        Ok(TestAdd::build(&mut args)?)
    } else if &cmd[..] == "say" {
        Ok(TestSay::build(&mut args)?)
    } else {
        Err("Invalid command!")
    }
}

pub trait Command {
    fn build(args: &mut impl Iterator<Item = String>) -> Result<Box<Self>, &'static str> where Self: Sized;
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct TestAdd {
    nums: Vec<i32>,
}

impl Command for TestAdd {
    fn build(args: &mut impl Iterator<Item = String>) -> Result<Box<Self>, &'static str> where Self: Sized {
        let nums: Vec<i32> = args 
            .filter_map(|n| {
                match n.parse::<i32>() {
                    Ok(x) => Some(x),
                    _ => None,
                   }
            }).collect();

        if nums.len() <= 0 { return Err("Missing number(s) to add!"); }

        Ok(Box::new(TestAdd { nums }))
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let sum: i32 = self.nums
            .iter()
            .sum();
        println!("{}", sum);

        Ok(())
    }
}

#[derive(Debug)]
pub struct TestSay {
    message: String,
}

impl Command for TestSay {
    fn build(args: &mut impl Iterator<Item = String>) -> Result<Box<Self>, &'static str> where Self: Sized {
        let message = match args.next() {
            Some(s) => String::from(s),
            None => return Err("Missing message to print!"),
        };

        Ok(Box::new(TestSay { message }))
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("{}", self.message);

        Ok(())
    }
}

//#[derive(Debug)]
//pub enum CommandKind {
//    TestSay(String),
//    TestAdd(Vec<i32>),
//}
//
//#[derive(Debug)]
//pub struct Command {
//    pub kind: CommandKind,
//}
//
//impl Command {
//    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Command, &'static str> {
//        // First item is always name of program, use next() to get rid of it
//        args.next();
//
//        let kind = match args.next() {
//            Some(cmd) => match Self::get_command_type(&cmd, args) {
//                Ok(kind) => kind,
//                Err(e) => return Err(e),
//            },
//            None => return Err("Missing a command!"),
//        };
//
//        Ok(Command { kind })
//    }
//
//    fn get_command_type(cmd: &str, mut args: impl Iterator<Item = String>) -> Result<CommandKind, &'static str> {
//        match cmd {
//            "add" => {
//                let nums: Vec<i32> = args 
//                    .filter_map(|n| {
//                        match n.parse::<i32>() {
//                            Ok(x) => Some(x),
//                            _ => None,
//                        }
//                    }).collect();
//
//                if nums.len() <= 0 { return Err("Missing number(s) to add!"); }
//
//                Ok(CommandKind::TestAdd(nums))
//            },
//            "say" => {
//                let message = match args.next() {
//                    Some(s) => String::from(s),
//                    None => return Err("Missing a message!"),
//                };
//
//                Ok(CommandKind::TestSay(message))
//            },
//            _ => return Err("Invalid command!"),
//        }
//    }
//}
