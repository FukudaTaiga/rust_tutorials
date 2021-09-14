// cargo new hello_cargo --bin -> generate Rust project with binary
//                       --vcs none -> project without git controll
//                       --lib -> library project 
// Cargo.toml in Rust is like package.json in javascript
// cargo build -> make Cargo.lock and target/debug/hello_cargo
// cargo run = cargo build && ./target/debug/hello_cargo -> for shorthand to build and run
// cargo check -> not build but check whether codes pass compiling or fail
// cargo build --release -> build with optimization
fn main() {
    println!("Hello, world!");
}
