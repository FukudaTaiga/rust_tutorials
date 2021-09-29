//sec 3
//const must be determined when compiled
//const can be declared in any scope
//const need type annotation
const _CONSTANT: u32 = 600;

fn main() {
    var_declare();
    shadow_and_mut();
    basic_scalar_type();
    multiple_tuple_and_array();
}

fn var_declare() {
    //`let` declare immutable var
    let x = 1;
    //x = 2 compile error
    println!("imut value: {}", x);
    let x = 2;
    //shadowing - redeclaration !!shadowing generate new variable
    println!("imut value: {}", x);

    //`let mut` declare mutable var
    let mut x = 5;
    println!("mut value: {}", x); //5
    x = 6;
    println!("mut value: {}", x); //6
}

fn shadow_and_mut() {
    //difference between shadowing and mut
    let spaces = "sssss";
    println!("spaces: {}", spaces);
    let spaces = spaces.len();
    println!("spaces len: {}", spaces); //these wont cause error

    let mut _spaces = "sssss"; //_ prefix disable a warning
                               //_spaces = _spaces.len(); //compile error
}

//types - scalar type (primitive) or multiple type
fn basic_scalar_type() {
    // scalar type
    // signed integer - i8~i64, isize(depending on OS)
    // unsigned - u8~u64, usize(depending on OS)
    // separating char - _
    // default - i32
    let _int: i64 = 2222_2222; //i64
    let _int = 57u8; //u8 suffix
    let _int = 0xff; //16
    let _int = 0o55; //8
    let _int = 0b1010; //2
    let _int = b'A'; //byte

    //scalar type
    //float - f32 or f64
    //default - f64
    let _float = 3.1415926535; //f64
    let _float: f32 = 3.1415926535; //f32

    //scalar type
    //bool - true or false
    let _b = true;
    let _b: bool = false;

    // scalar type
    // char - quated by '' (not "", used by string)
    // implemented by unicode - many character is valid
    let _c = 'c';
}

fn multiple_tuple_and_array() {
    // basic multiple type
    // tuple
    let tuple = (1, 2, '3');
    println!("tuple's second value: {}", tuple.1);
    let tuple: (f32, f64, char) = (4.0, 2.0, '6');
    let (_x, y, _z) = tuple; //decomposed with pattern matching
    println!("tuple's second value: {}", y);

    // basic multiple type
    // array - fixed length
    // in contrast, vector has non-fixed length
    let _array = [0, 1, 2, 3];
    let array: [i32; 4] = [1; 4]; //[1, 1, 1, 1]
    println!("array's second value: {}", array[1]);
    //println!("array's second value: {}", array[4]); !runtime error - out of index
}
