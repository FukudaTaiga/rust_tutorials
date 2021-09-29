#![allow(unused)]
//Sec 18
//pattern is notation that matches to some type structure
//PATTERN is created by
//  literall
//  destructed array, enum, structure, tuple
//  variables
//  wild card _
//  placeholder

fn main() {
    pattern_destruction();
    all_patterns();
    pattern_ref();
    match_guard();
    at_binding();
}

fn pattern_destruction() {
    println!("all pattern destruction!");
    //match arms
    let option = Some(0);
    let b = true;
    let result: Result<&str, &str> = Ok("result");

    println!("with match {{ PATTERN = EXPRESSION }}");
    let matched = match option {
        Some(_) => 1,
        None => 0,
    };
    println!("{}", matched);

    println!("with if let PATTERN = EXPRESSION");
    let matched = if let Some(_) = option {
        1
    } else if let Ok(a) = result
    //invalid: let Ok(a) = result && a.len() == 0
    {
        //a begin valid in this scope
        if a.len() == 0 {
            2
        } else {
            -2
        }
    } else if b {
        3
    } else {
        0
    };
    println!("{}", matched);

    println!("while let PATTERN = EXPRESSION");
    let mut stack = Vec::<i32>::new();
    for i in 0..3 {
        stack.push(i);
    }
    while let Some(v) = stack.pop() {
        println!("while let: Some({})", v);
    }

    println!("for PATTERN in ...");
    let v = vec!['a', 'b', 'c'];
    for (idx, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, idx);
    }

    println!("let PATTERN = EXPRESSION");
    let (x, y, z) = ("x", "y", "z");
    println!("destructed ({},{},{})", x, y, z);
    //let (x, y) = ("x", "y", "z"); compile error
    let (a, b, c, ..) = (1, 2, 3, 4, 6, 5);

    println!("fn __name__(PATTERN: __type__)");
    //below notation is also usable in closure
    fn dest(&(x, y): &(i32, i32)) {
        println!(
            "these value is destructed from tuple ({}, {}) in function arguments",
            x, y
        );
    }
    dest(&(1, 2));

    //Refutability - if let,...
    println!(
        "
  if there may be uncatchable pattern, PATTERN is usable in `if let` or `while let`
  if there is no uncatchable pattern, PATTERN is usable in `fn arg`, `let ...`, `for`
  "
    );

    println!("----------------------------------------");
}

fn all_patterns() {
    println!("all pattern usage");

    println!("literalls");
    let x = 3;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("other"),
    };

    println!("named match");
    let x = Some(5);
    let y = 10;
    println!("x: {:?}, y: {}", x, y);
    match x {
        Some(50) => println!("50!"),
        Some(y) => println!("inner value x: {}", y),
        _ => println!("should reach in None case: {:?}", x),
    };
    println!("x: {:?}, y: {}", x, y);

    println!("or match");
    let x = 5;
    match x {
        1 | 2 => println!("first arm"),
        3 | 4 | 5 => println!("second arm"),
        _ => println!("anything"),
    };

    println!("range match - deprecated");
    match x {
        1...5 => println!("first range"),
        3...10 => println!("second range"),
        _ => println!("uncatched range"),
    };

    println!("destruct structure");
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 100, y: 150 };
    let Point { x: a, y: b } = p;
    println!("original {:?}", p);
    println!("destructed a: {}, b: {}", a, b);
    let Point { x, y } = p;
    println!("shorthand destructed  {}, {}", x, y);
    match p {
        Point { x, y: 100 } => unreachable!(),
        Point { x: 100, y } => (),
        Point { x, y } => unreachable!(),
    };

    //such destruct is also effective for enum, reference

    //ignoring
    {}

    println!("----------------------------------------");
}

fn pattern_ref() {
    println!("pattern ref");
    println!("TODO");
    println!("----------------------------------------");
}

fn match_guard() {
    println!("match guard");
    println!("TODO");
    println!("----------------------------------------");
}

fn at_binding() {
    println!("at binding");
    println!("TODO");
    println!("----------------------------------------");
}
