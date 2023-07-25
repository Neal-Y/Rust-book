use minigrep as dir;
use std::env;
use std::process;

fn main() {
    let config = dir::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("parsing problem: {}", err);
        process::exit(1);
    });
    if let Err(e) = dir::run(config) {
        eprintln!("failed to run : {}", e);
        process::exit(1);
    };
}
