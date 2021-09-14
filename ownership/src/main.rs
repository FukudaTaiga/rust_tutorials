// Section 4 Ownership
// memory: stack or heap
// stack - high velocity, fixed size, LIFO and handling with push or pop
// heap  - slow, variable length, ramdom access and allocating on the heap
//         = find enough space and return a pointer
//         used if the required size can't be determined on the compile or need variable length
// ownership is third choice of handling heap - first is by a programmer, second is gc
// three rule: 1. any value is owned by `owner` variables
//             2. whenever `owner` is exact one
//             3. abondon if `owner` disappear from the current scope

fn main() {
  // rust adopt value substitution on prelude(primitive) - deep copy (= shallow copy)
  //           reference substitution on the others - shallow copy
  // check! - Copy trait
  let mut n1 = 5;
  let n2 = n1;
  n1 = 8;
  println!("n1: {}", n1); // 8 n1 still available
  println!("n2: {}", n2); // 5

  let s1 = String::from("hello"); // allocate on the heap for variable
  let s2 = s1;
  // s1 is not available any more
  // cuz its value already moved to s2
  //println!("s1: {}", s1); compile error
  println!("s2: {}", s2);

  let s3 = take_copy(s2); // s2's value owner moved to take_copy function
  //let s4 = s2; compile error - s2 already unavailable
  println!("s3: {}", s3);
  take_ref(&s3); //borrow with function call passing ref of variable
  println!("alive! {}", s3);
  slice();
}

fn take_copy(str: String) -> String {
  dummy();
  let mut s = str.clone(); // Strings deep copy
  s.push_str(" world");
  s //s owner moved to caller from this function
}

fn take_ref(sptr: &String) { // <- should be &mut String if there is a need to mutate
  //sptr.push_str("fail"); compile error - ref is immutable by default
  println!("length : {}", sptr.len());
}

fn dummy() {
  let _dum = String::from("aaa");
  // Rust calls special function `drop()` to free unnecessary memory (for _dum there)
  // its not called if returning _dum cuz _dum owner move to caller
}

// refference doesnt have `owner`!
// refference rule:
//   1. you can take `1 mutable ref` or `some immutable refs`
//   2. ref must be always available
//
// two kind of ref - ref and slice

fn slice() {
  let string: String = String::from("hello world");
  let _hello:&str = &string[..4];
  let _world:&str = &string[5..];
  let lo_wo: &str = &string[3..7];
  println!("slice: {}", lo_wo);

  let arr = [0, 1, 2, 3, 4];
  let _ar = &arr[..2]; // [0, 1]
  let _rr = &arr[3..]; // [3, 4]
  let r = &arr[1..3];  // [1, 2]
  println!("sliced array[1]: {}", r[1]);
}
