use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem with arguments: {err}");
        process::exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}
