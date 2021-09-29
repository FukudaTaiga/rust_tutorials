//sec 19
//  Unsafe Rust
//  advanced lifetime
//  advanced trait
//  advanced type
//  advanced function and closure
mod trait_adv;
mod type_adv;
mod unsafe_rust;
mod function_adv;
mod rust_macro;

fn main() {
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
    trait_adv::trait_adv_demo();
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

    println!("advanced type!");
    type_adv::type_adv_demo();
    println!("----------------------------------");

    println!("advanced function!");
    function_adv::function_adv_demo();
    println!("----------------------------------");

    println!("macro!");
    println!("
----------------------------------------------------------------------------------------
|Why macro exists?                                                                     |
|macro is a kind of meta programming                                                   |
|it will be expanded before compiler interpretation                                    |
|it reduce code amount, but main difference from function is                           |
| 1 - can take a variable number arguments, as opposed to function take fixed          |
| 2 - executed before compile, so can do complex like implements trait for given type  |
| 3 - must define macro before called, as opposed to funciton can anywhere             |
|to short, macro is code generation                                                    |
|   = get code and provide required as as `proc_macro::TokenStream`                    |
|macro is very powerfull functionaity, but complex...                                  |
|think trade off before use                                                            |
----------------------------------------------------------------------------------------\
");
    rust_macro::macro_demo();
    println!("----------------------------------");
}
