#![allow(dead_code)]
//Why Unsafe Rust need?
//  static analysis is conservative --- but computer is unsafe originally
//  for low level operation, like related to OS, it is necessary
//  unsafe codes tell compiler "I know what i do, trust me"

pub fn unsafe_demo() {
    //unsafe rust provides 5 action called "unsafe superpower"
    //  1, Dereference a raw pointer
    //  2, Call an unsafe function or method
    //  3, Access or modify a mutable static variable
    //  4, Implement an unsafe trait
    //  5, Access fields of unions -- for interface with union in C code
    //Conversally, Unsafe Rust `Doesn't` turn off borrow checker, and any other
    //Unsafe doensnt means the code block is necessary unsafe

    raw_pointer();
    call_unsafe_fn();
    safe_wrapper_of_unsafe();
    use_ffi();
    static_var();
    trait_unsafe();
}

fn raw_pointer() {
    println!("raw pointer:");
    //Differing from reference or smart pointer,
    //  - multiple immutable or mutable pointer can exist at the same time
    //  - no garantee of valid access
    //  - nullable
    //  - no auto typing

    //raw pointer is typed as *const T (imm) or *mut T

    //new - it can be created in safe rust
    let mut num = 5; // from reference
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    //let rv = *r1;   the ristriction is `dereferencing`

    let address = 0x012345usize;
    let r = address as *mut i32; //raw pointer indicates some memory

    //deref
    unsafe {
        println!("unsafe pointer const: {}", *r1);
        println!("unsafe pointer mut: {}", *r2);
    }

    use std::slice;

    let _danger_slice = unsafe {
        slice::from_raw_parts_mut(r, 100000) //no guarantee the values is valid i32
    }; //use danger_slice as if its valid results in undefined behavior
}

fn call_unsafe_fn() {
    unsafe fn danger() {
        println!("Hello, Unsafe World");
    }

    //danger(); illegal
    unsafe {
        danger();
    }
}

fn safe_wrapper_of_unsafe() {
    println!("safe wrapping of unsafe");
    use std::slice;
    //create safe abstruction over unsafe is generall usecase
    fn split_string_mut(tar: &mut [u8], pos: usize) -> (&mut [u8], &mut [u8]) {
        let len = tar.len();
        let ptr = tar.as_mut_ptr(); //get raw pointer, *mut u8, in this case

        assert!(pos <= len);

        unsafe {
            //(tar[..pos].as_bytes_mut(), tar[pos..].as_bytes_mut()) //--- borrowing two mutable reference is forbuidden
            (
                slice::from_raw_parts_mut(ptr, pos), //create slice from raw pointer and length
                slice::from_raw_parts_mut(ptr.add(pos), len - pos),
            )
        }
    }

    let tar = &mut [b'a', b'b', b'c', b'd', b'e'];
    let (pre, suf) = split_string_mut(tar, 2); //above function is safe and no need to use unsafe block though it includes unsafe code
    println!("splitting result: {:?}, {:?}", pre, suf);
}

fn use_ffi() {
    println!("Foreign Function Interface, FFI");

    //`extern` keyword provides interface of ffi from other language
    extern "C" {
        //the part `"C"` is called application binary interface (ABI). "C" is most common and follows C's ABI
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("absolute of -2 according to C is {}", abs(-2));
    }

    //`extern` also allows other language to use Rust function, unsafe is not required for this usage
    #[no_mangle] //Mangling is a way for compilers to handle function internally, avoid to consider it deeply now
    pub extern "C" fn call_from_c() {
        println!("Hello C from Rust World");
    }
}

static HELLO_WORLD: &str = "Hello, World";
static mut COUNTER: i32 = 0;

fn static_var() {
    //static that declare 'static lifetime variable is like const, but there are some differences
    //  1. static var have fixed address in memory and always accessed on same address, constant can duplicate their data
    //  2. static can be mutable, and its modification is unsafe: race condition will occur in multi thread
    //    - in such case, consider using thread safe tequniques

    println!("Static {}", HELLO_WORLD);

    //mutable static var is only used in unsafe block
    fn add() {
        unsafe {
            COUNTER += 1;
        }
    }

    for _ in 0..10 {
        unsafe {
            println!("static mut COUNTR is {}", COUNTER);
        }
        add();
    }
}

fn trait_unsafe() {
    println!("unsafe trait");
    //trait become unsafe if its at least one methods have unsafe invariant
    unsafe trait Foo {
        unsafe fn danger(&self);
    }
    unsafe impl Foo for i32 {
        unsafe fn danger(&self) {
            println!("Danger Trait..., {}", self);
        }
    }

    unsafe {
        1.danger();
        2.danger();
        3.danger();
    }
}
