//sec 20

//1. learn TCP, HTTP
//2. listen TCP with socket
//3. parse HTTP request structure
//4. response HTTP
//5. emphasis server's throughput with thread pool

/*
this section refers to a Rust Compiler's problem
and complex life timeimplementation and so on
Read more carefully, its not enough
*/
extern crate multi_thread_server;

use std::env;

fn main() {
    //to give arg with `cargo run`, use `cargo -- options` not to recognized as `cargo run`'s option
    let mut args = env::args();
    args.next();
    let mode = match args.next() {
        Some(q) if q == "--single".to_string() => false,
        Some(_) => true,
        None => true,
    };

    if mode {
        multi_thread_server::run();
    } else {
        multi_thread_server::run_single_thread();
    }
}
