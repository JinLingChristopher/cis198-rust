use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Config::new(&args).unwrap_or_else(
        |err| {
            println!("Problem parsing argument: {}", err);
            process::exit(1);
        }
    );

    println!("search for `{}` in the source `{}`", conf.target, conf.filename);
    if let Err(e) = conf.run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
