use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(option) => {
            if &option[..] != "skip" {
                closure();
            }
        }
        None => closure(),
    }
    iterator();
    summary();
}

fn closure() {
    println!("closure!");
    // anonymous function - originated in Functional Programming

    //notation
    let _c1 = |a: u32| {
        let b = a + a;
        b * b
    };
    let _c2 = |a: u32, b: u32| a + b;
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    //let add_one_v3 = |x| { x + 1 }; through compile if used
    //let add_one_v4 = |x| x + 1 ; through compile if used

    fn difference_between_closure_and_function() {
        //closure can capture environment
        // this is closure's overhead - take care
        {
            let x = 4;
            let equal_to_x = |z| z == x;
            let y = 4;
            assert!(equal_to_x(y));
        }
        //function can't
        {
            let x = 4;
            //fn equal_to_x(z: i32) -> bool { z == x } compile error
            let y = 4;
            //assert!(equal_to_x(y));
        }
    }
    //closure can use environment in three ways
    //FnOnce - take ownership, implemented by any closure
    //  consumes the variables it captures from its enclosing scope, known as the closure’s environment.
    //  To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined.
    //  The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
    //FnMut - take mutable reference
    //  can change the environment because it mutably borrows values.
    //Fn - take immutable reference
    //  borrows values from the environment immutably.
    //FnOnce < FnMut < Fn : severity

    //type inference
    let id = |x| x;
    let _ex = id("aaa"); //id is infered type Fn(&str) -> &str
                         //let _ex = id(5); //compile error cuz i32
                         //such type differs if same process function
    let id2 = |x| x;
    let _ex = id2(1);

    //force for closure to move value (take arguments ownership)
    {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        //println!("can't use x here: {:?}", x);
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }

    //memorization, lazy val
    //closure trait
    struct Cacher<T, S>
    where
        T: Fn(S) -> S,
        S: std::cmp::Eq + std::hash::Hash,
    {
        calculation: T, //Fn, FnMut, FnOnce - closure trait
        values: HashMap<S, S>,
    }
    impl<S, T> Cacher<T, S>
    where
        T: Fn(S) -> S,
        S: std::cmp::Eq + std::hash::Hash + Copy,
    {
        fn new(calc: T) -> Cacher<T, S> {
            Cacher {
                calculation: calc,
                values: HashMap::new(),
            }
        }

        fn value(&mut self, arg: S) -> S {
            match self.values.get(&arg) {
                Some(v) => *v,
                None => {
                    let v = (self.calculation)(arg);
                    self.values.insert(arg, v);
                    v
                }
            }
        }
    }

    fn casher_demo() {
        let mut closure = Cacher::new(|x: i32| {
            println!("you have this message only if the input is first time");
            x + 1
        });

        println!("type some Integer, or enter to quit");
        loop {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();

            let s = s.trim();
            if s.is_empty() {
                println!("quit");
                break;
            }

            let i = match s.parse::<i32>() {
                Ok(i) => i,
                Err(_) => {
                    println!("type integer");
                    continue;
                }
            };
            closure.value(i);
        }
    }

    fn use_case() {
        let specified = 10;
        let random = 5;

        fn simulate_expensive_calc(intensity: u32) -> u32 {
            println!("calc...expensive");
            thread::sleep(Duration::from_secs(2));
            intensity
        }
        fn generate(intensity: u32, random: u32) {
            //to avoid unecessary calculation, one approach is
            //let expensive = simulate_expensive_calc(intensity);
            //  problem -> even if random == 3, expensive will be calc
            if intensity < 25 {
                println!("simulate: First, {}", simulate_expensive_calc(intensity));
                println!("simulate: Second {}", simulate_expensive_calc(intensity));
            } else {
                if random == 3 {
                    println!("simulate: nothing");
                } else {
                    println!("simulate: Do {}", simulate_expensive_calc(intensity));
                }
            }
        }

        fn generate_with_closure(intensity: u32, random: u32) {
            let mut expensive_closure = Cacher::new(|num| {
                println!("calc...expensive");
                thread::sleep(Duration::from_secs(2));
                num
            });

            if intensity < 25 {
                println!("simulate: First, {}", expensive_closure.value(intensity));
                println!("simulate: Second {}", expensive_closure.value(intensity));
            } else {
                if random == 3 {
                    println!("simulate: nothing");
                } else {
                    println!("simulate: Do {}", expensive_closure.value(intensity));
                }
            }
        }

        generate(specified, random);
        generate_with_closure(specified, random);
    }

    use_case();
    casher_demo();
    println!("----------------------------------------");
}

fn iterator() {
    //iterator is lazy, must be consumed
    println!("iterator!");
    //iterator
    //pub trait Iterator {
    //  type Item;
    //  fn next(&mut self) -> Option<Self::Item>;
    //  // methods with default implementations elided
    //}

    let v = vec![1, 2, 3];
    let s = String::from("one, two, three");

    //create, but nothing done at this time
    let v_itr = v.iter();
    let mut s_chars = s.chars();
    let s_bytes = s.bytes();
    //  iterator returning value instead of ref
    let v = vec![1, 2, 3];
    let v_val_itr = v.into_iter();
    //  iterator returning mutable ref instead of immutable ref
    let mut v = vec![1, 2, 3];
    let v_mut_itr = v.iter_mut();

    //access
    //  with for
    for v in v_itr {
        println!("accessed with for: {}", v);
    }
    //  with next() - return immutable ref of the value
    //                require variable mutable (= mutate the internal state and consume it)
    loop {
        let val = s_chars.next();
        match val {
            Some(i) => {
                println!("accessed with next: Some({})", i);
                assert_eq!(val, Some(i));
            }
            None => {
                println!("accessed with next: None");
                assert_eq!(val, None);
                break;
            }
        }
    }

    //consumer adoptor
    //  - Iterator method calling next() internally
    {
        let v = vec![1, 2, 3];
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 6);
        println!("sum of v is: {}", sum); //v.iter()'s value consumed
    }

    //iterator adoptor
    //  - Iterator method transform itself into another iterator (but still lazy)
    {
        let v = vec![1, 2, 3];
        let mapped_sum: Vec<_> = v.iter().map(|x| x + 1).collect();
        println!("mapped v: {:?}", mapped_sum);
    }
    println!("----------------------------------------");
}

fn summary() {
    println!("summary!");

    //closure with iterator
    {
        let env_val = 4;
        let v = vec![1, 10, 3, 6, 87, -1, 0, 100];
        println!("v: {:?}", v);
        let v: Vec<_> = v.iter().filter(|x| env_val < **x).collect();
        println!("filtered v: {:?}", v);
    }

    struct RandomWalk {
        internal: i32,
    }
    impl RandomWalk {
        fn new() -> RandomWalk {
            RandomWalk { internal: 0 }
        }

        const MIN: i32 = -10;
        const MAX: i32 = 10;
    }
    impl Iterator for RandomWalk {
        //able to call any Iterator method with default impl as long as defining next()
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let dice = rand::thread_rng().gen_range(0, 2);

            if dice == 0 {
                self.internal -= 1;
            } else {
                self.internal += 1;
            }

            if self.internal < RandomWalk::MAX && RandomWalk::MIN < self.internal {
                Some(self.internal)
            } else {
                None
            }
        }
    }

    //use Iterator methods for original itr
    {
        let rw = RandomWalk::new();
        println!("Random Walk: {}", rw.sum::<i32>());
    }

    println!("----------------------------------------");
}
