#![allow(dead_code)]
use std::fmt;
use std::ops::{Deref, DerefMut};

pub fn adv_trait_demo() {
    associated_type();
    //default type arg is mainly used for two purpose
    //  1. extends code without breaking existing code
    //  2. allow customization in very specific case --- Add trait case
    default_type_arg();
    fully_qualified_syntax();
    super_trait();
    new_type();
}

fn associated_type() {
    println!("associated type");
    //associated type is like a generics
    //  it claims its implementation to explicit the type
    //  it enables to define functions having some type parameter without generics
    //difference againt generics - if with generics, there may be multiple implementation for some type.
    //ass type doesn't allow such impl and reduce typing amount to have type parameter

    trait MyTr1 {
        type A;
        fn ex1(&self) -> Self::A;
    }
    trait MyTr2<A> {
        fn ex2(&self) -> A;
    }

    impl MyTr1 for i32 {
        type A = String;
        fn ex1(&self) -> Self::A {
            String::from("trait with associated type")
        }
    }

    impl MyTr2<String> for i32 {
        fn ex2(&self) -> String {
            String::from("trait with generics: String")
        }
    }
    impl MyTr2<char> for i32 {
        fn ex2(&self) -> char {
            'a'
        }
    }

    println!("Hello, {}", 1.ex1());
    //need type annotation
    println!("Hello, {}", MyTr2::<String>::ex2(&1));
    println!("Hello, {}", MyTr2::<char>::ex2(&1));
}

fn default_type_arg() {
    println!("default type argument");
    //defaut type arg applied if not provided
    //a good usecase is overloading
    //this is a Add trait
    // trait Add<Rhs=Self> {
    //   type Output;

    //   fn add(self, rhs: Rhs) -> Self::Output;
    // }
    use std::ops::Add;

    #[derive(Debug)]
    struct Point(i32, i32);
    //without type parameter Rhs
    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Point(self.0 + rhs.0, self.1 + rhs.1)
        }
    }
    //with type parameter Rhs
    impl Add<(i32, bool)> for Point {
        type Output = Point;

        fn add(self, rhs: (i32, bool)) -> Self::Output {
            if rhs.1 {
                Point(self.0, self.1 + rhs.0)
            } else {
                Point(self.0 + rhs.0, self.1)
            }
        }
    }

    let p = Point(3, 4);
    println!("{:?}", p);
    let p = p + Point(1, 2);
    println!("{:?}", p);
    let p = p + (10, false);
    println!("{:?}", p);
    let p = p + (-10, true);
    println!("{:?}", p);
}

fn fully_qualified_syntax() {
    println!("full path syntax");
    //no complexity, give no explanation
    //  if same named method exists, the method defined in struct directly is prior

    trait T1 {
        fn hello(&self);
        fn fullpath();
    }
    trait T2 {
        fn hello(&self);
    }

    struct A;
    impl A {
        fn hello(&self) {
            println!("Hello by struct A");
        }
    }
    impl T1 for A {
        fn hello(&self) {
            println!("Hello by trait T1");
        }
        fn fullpath() {}
    }
    impl T2 for A {
        fn hello(&self) {
            println!("Hello by trait T2");
        }
    }

    struct FullPath;
    impl T1 for FullPath {
        fn hello(&self) {}
        fn fullpath() {
            println!("This is full path syntax, related function");
        }
    }

    let a = A;
    a.hello();
    A::hello(&a); //same as `a.hello();`
    T1::hello(&a);
    T2::hello(&a);

    //full path
    //<Type as Trait>::function(receiver_if_method, next_arg, ...);
    <FullPath as T1>::fullpath(); //use Fullpath's T1 related function impl
}

fn super_trait() {
    println!("super trait");
    //trait assuming other trait
    trait Decorate: fmt::Display {
        fn print_dec(&self) {
            let out = self.to_string(); //use fmt::Display's method
            let len = out.len() + 2;

            println!("{}", "*".repeat(len));
            println!("*{}*", out);
            println!("{}", "*".repeat(len));
        }
    }
    impl Decorate for String {}

    let w = "decorated string".to_string();
    w.print_dec();
}

fn new_type() {
    println!("new type pattern");
    //to avoid trait's orphan rule, new type pattern is usefull
    //For example, A is struct and T is trait defined externally
    //  and we want to implement T for A
    //  define new type W wrapped A and implement T for W
    //        T -> W(A)
    //  to use A's method from W,
    //  1 - define all required intermidiate method calling A's
    //  2 - implement Deref and use it as smart pointer
    //new pattern has no runtime overhead

    //in this case A is Vec<String>, T is fmt::Display
    struct Wrapp(Vec<String>);
    impl fmt::Display for Wrapp {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "wrapped: [{}]", self.0.join(","))
        }
    }
    impl Deref for Wrapp {
        type Target = Vec<String>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl DerefMut for Wrapp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    let mut w = Wrapp(vec![
        "aaa".to_string(),
        "bbb".to_string(),
        "ccc".to_string(),
    ]);
    println!("{}", w);
    w.push("ddd".to_string());
    println!("{}", w);
}
