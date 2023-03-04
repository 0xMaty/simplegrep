use simplegrep::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(err) = simplegrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
