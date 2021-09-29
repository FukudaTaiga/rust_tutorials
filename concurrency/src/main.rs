//sec 16
//multi thread controll
//  1. set up thread and run multi code simultaneously
//  2. message concurrency that each chanel send a message to other
//  3. state shareness of each thread
//  4. Sync and Send trait
//multi thread programming is complicated and has potentially unique problems, like
//  - race condition
//  - dead lock
//  - bug ocurring on specified condition and hard to reproduce
//thread model - 1:1, M:N
//Rust adopts 1:1 model for performance reason (a bit runtime), but M:N crate exists
//this means the maximum number of Rust threads is depends on OS's property
mod channel;
mod mutex;
mod usage;

fn main() {
    println!("usage!");
    usage::usage();
    println!("------------------------------------");

    println!("channel!");
    channel::channel();
    println!("------------------------------------");

    println!("mutex!");
    mutex::mutex();
    println!("------------------------------------");

    println!("Send/Sync trait");
    println!("
    almost all of Rust's concurrency functionality is a part of std, not a Language notation
    but, two concepts are embedded in Rust - std::marker's traits Send/Sync
    Mutex/Arc implement them, RefCell/Rc don't

    Send
    Send marker indicates ownership of the value typed as Send trait is transferred between thread safely
    almost all type like Primitive, the type composed entirely of Send types
        is (automatically) marked as Send aside from some exception of Rc, Cells like RefCell, raw pointer

    Sync
    Sync marker indicates it is safe for the type impl Sync to be referenced from multiple threads
    in other words, any type T is Sync <=> &T is Send

    implementing Send/Sync manually is unsafe and require unsafe Rust --- see Rustnomicon (https://doc.rust-lang.org/nomicon/index.html)
    ");
    println!("------------------------------------");
}
