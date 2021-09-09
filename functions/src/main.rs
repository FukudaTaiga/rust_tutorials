use rand::Rng;

//statement: end with ; like let x = ...;, loop {}; - never return any value
//expression: value, function call, macro call, block({}), if - never add ;

//need type annotation for arg (and return value if exists)
fn _add(x: i32, y: i32) -> i32 {
  return x + y;
}

//return last asserted expression like scala
fn _five() -> i64 {
  5 //if ; added, compile error
}

fn main() {
  after_main(-100);
  after_main(10);
  after_main(1000);
  let _x = after_main(10); // x is (), and printed "x is under 100"
  loops();
  println!("fibonutti 60 = {}", fib(20));
}

//function can be declared anywhere
fn after_main(x: i64) {
  //`if` never have other than bool unlike js
  let log = if x < 0 {
    "x is negative"
  } else if x < 100 {
    "x is under 100"
  } else {
    "x is over 100" //if 6, compile error cuz can't determine log type
  }; //never have multiple type
  println!("{}", log);
}

fn loops() {
  //`loop` means while(true)
  loop {
    let r = rand::thread_rng().gen_range(1, 101);
    println!("random value is {}", r);
    if r < 50 {
      continue;
    } else {
      break;
    }
  }
  //while
  let mut i = 10;
  while 0 < i {
    println!("while i: {}", i);
    i = i - 1;
  }

  //for - iterator
  let a = [1, 5, 6, 3, 10];
  for el in a.iter() {
    println!("iterative for {}", el);
  }

  println!("i {}", i);
  //for - range
  //inner variable(i here) seem to be not affected by other scope variable
  for i in 1..5 {
    println!("range for {}", i);
  }
  println!("i {}", i);
}

fn fib(n: u32) -> u32 {
  let mut x = 0;
  let mut y = 1;
  if n == 0 {
    return x;
  } else if n == 1 {
    return y;
  } else {
    for _i in 0..n - 1 {
      let temp = y;
      y = x + y;
      x = temp;
    }
    return y;
  }
}
