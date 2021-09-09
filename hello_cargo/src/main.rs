// cargo new hello_cargo --bin により生成.
// Cargo.toml が package.json にあたる.
// cargo buildで
// Cargo.lock
// target/debug/hello_cargo
// が生成される.
// -> cargo run = cargo build && ./target/debug/hello_cargo のみでok.
// コンパイルが通るかのみであれば cargo check.
// リリースは cargo build --release (最適化込み)
fn main() {
    println!("Hello, world!");
}
