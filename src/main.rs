use std::env;
use std::process;

use grimoire::Command;

fn main() {
    println!("Hello, world!");

    let command = Command::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Error! {e}");
        process::exit(1);
    });

    dbg!("{:?}", command.kind);
}
