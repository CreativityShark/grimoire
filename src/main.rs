use std::env;
use std::process;
use grimoire::spell;

fn main() {
    let cmd = grimoire::build_command(env::args()).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        process::exit(1);
    });

    if let Err(e) = grimoire::run(cmd) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
