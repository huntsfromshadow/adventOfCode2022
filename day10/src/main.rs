use std::env;
use std::process;

mod puzzle_lib;
use day10::Config;
use puzzle_lib::run;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });

    println!("Starting\n\n");

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    println!("\n\nEnding");
}