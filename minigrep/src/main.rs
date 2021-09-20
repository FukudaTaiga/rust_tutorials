extern crate minigrep;

use std::env; //process given argument with execution `cargo run (...args)`
use std::process;
use minigrep::Config;

//divide program into main.rs and lib.rs following steps
//  Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
//  As long as your command line parsing logic is small, it can remain in main.rs.
//  When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
//then, main.rs have responsibilities like
//  Calling the command line parsing logic with the argument values
//  Setting up any other configuration
//  Calling a run function in lib.rs
//  Handling the error if run returns an error

fn main() {
    //get args as Vec - this will panic if args including invalid unicode
    //to handle such value, use `envs::args_os()` <- more complex
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| { //closure - anonymous function
      eprintln!("Problem parsing arguments: {}", err); //to stderr
      process::exit(1);
    });

    //println!("Serching {} in {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) { //Result<(), ...> is unused when Ok -> if let
      eprintln!("Application error: {}", e);
      process::exit(1);
    }
}
