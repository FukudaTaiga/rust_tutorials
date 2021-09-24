pub mod other_child;

pub fn other_print() {
    other_child::other_child_print();
    println!("this function imported from other.rs");
}
