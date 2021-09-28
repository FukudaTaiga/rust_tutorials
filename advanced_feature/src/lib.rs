mod trait_adv;
mod type_adv;
mod unsafe_rust;

pub fn run() {
    println!("unsafe!");
    println!(
        "
  ---------------------------------------------------------------------------
  |unsafe code is powerfull, and have a probability of undefined behavior.  |
  |additionally, compiler can't give helps like type checker, memory safety,|
  |so the codes may be something trickier.                                  |
  |but explicit unsafe block will help tracking the bugs, use it carefully  |
  ---------------------------------------------------------------------------\
  "
    );
    unsafe_rust::unsafe_demo();
    println!("----------------------------------");

    println!("advanced trait!");
    trait_adv::adv_trait_demo();
    println!("----------------------------------");

    println!(
        "
  -----------------------------------------
  |new type pattern is very usefull.      |
  |1. in trait extention                  |
  |2. in type abstruction, API translation|
  |3. in simple encapsulation like OOP    |
  -----------------------------------------\
  "
    );
}
