use std::env;
use std::process;

use mj_minigrep::Config;

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem processing arguments {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);

    println!("In file {}", config.file_path);

    if let Err(e) = mj_minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
