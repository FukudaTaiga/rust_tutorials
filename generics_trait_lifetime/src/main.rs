#![allow(unused)]
//Sec 10

fn main() {
    demo_generics();
    demo_trait();
    demo_lifetime();
    summary();
}

fn demo_generics() {
    println!("generics!");
    //Generics is a general poworful approach to abstruct code
    //it can be used many data structure
    // - like function signature, struct, enum

    //in function signature
    //generics need type variable `<T>, <T, U>, ...`
    //sometimes add trait bound
    fn calc_largest<T: std::cmp::PartialOrd + Copy>(seq: &[T]) -> T {
        let mut largest = seq[0];

        for &el in seq {
            if largest < el {
                largest = el;
            }
        }

        largest
    }

    let v = vec![3, 1, 10, 2, -1];
    println!("v is {:?}, largest is {:?}", v, calc_largest(&v));
    let v = vec!["svds", "dscaa", "avdsv", "x"];
    println!("v is {:?}, largest is {:?}", v, calc_largest(&v));

    //in structure
    mod structure {
        #[derive(Debug)]
        pub struct GenericsStruct<T, U> {
            x: T,
            y: T,
            z: U,
        }
        //T, U related to structure type variable
        impl<T, U> GenericsStruct<T, U> {
            pub fn new(x: T, y: T, z: U) -> GenericsStruct<T, U> {
                GenericsStruct { x: x, y: y, z: z }
            }

            //V, W only related to this method
            pub fn _mixup<V, W>(self, other: GenericsStruct<V, W>) -> GenericsStruct<T, W> {
                GenericsStruct {
                    x: self.x,
                    y: self.y,
                    z: other.z,
                }
            }
        }
        //only defined for the pattern of T = i32, U = f64
        impl GenericsStruct<i32, f64> {
            pub fn calc_distance(&self) -> f64 {
                ((self.x as f64).powi(2) + (self.y as f64).powi(2) + self.z.powi(2)).sqrt()
            }
        }

        //dividing with trait bounding - should also search 'blanket implementation'
        impl<T: std::fmt::Display + PartialOrd, U> GenericsStruct<T, U> {
            pub fn large_x_y(&self) {
                if self.x < self.y {
                    println!("y is larger than x");
                } else {
                    println!("x is larger than y");
                }
            }
        }
    }

    let gen = structure::GenericsStruct::new(5, 3, 4.0);
    // GenericsDemo {x: 5, y: 3.0, z: 4.0}; compile error cuz x type infer i32, but y isn't
    println!("{:?}", gen);
    println!("method call calc_distance: {}", gen.calc_distance());
    gen.large_x_y(); // x is larger ...
    let gen = structure::GenericsStruct::new(3, 5, 4); //no problem cuz its a special case that T = U
    println!("{:?}", gen);
    //println!("method call calc_distance: {}", gen.calc_distance());
    // - compile error cuz T = U = i32
    gen.large_x_y(); // y is larger ...

    //in enum
    enum _GenericsEnum<A, B> {
        W(A, B),
        X(A),
        Y(B),
        Z,
    }

    //how about code performance with generics -> no cost
    // powered by monomorphization
    //  - in compile, replace all generics applied codes by concrete type code
    // there are some work in compile, but no cost in runtime

    println!("----------------------------------------");
}

fn demo_trait() {
    println!("trait!");
    //Trait is like interface in java, trait in scala
    //it can be deal with as if type like struct, enum
    //but, keep it in mind that trait is kinda restriction for type
    pub trait Pronunciation {
        //define common behavier
        //with default implementation
        // it is not able to call default by class overriding
        fn common_process() -> String {
            "default process".to_string()
        }
        //without default - force class to implement this
        fn required_process() -> String;
        fn value(&self) -> &str;
    }
    //coherence, especially orphan rule in trait
    // - limitation that implementation of external class for external trait is forbidden
    //   that is because compiler become disable
    //    to determine which impl should be used in runtime (if orphan rule violate)
    // impl std::cmp::PartialOrd for String {} //- compile error

    struct Vowel<'a> {
        value: &'a str,
    }
    impl Vowel<'_> {
        fn hello(&self) {
            println!(
                "hello!, Im {}. value is {} starting with vowel",
                Vowel::required_process(),
                self.value
            );
            println!("common process is {}", Vowel::common_process());
        }
    }
    //trait implementation for Vowel
    impl Pronunciation for Vowel<'_> {
        fn required_process() -> String {
            "class Vowel".to_string()
        }

        fn value(&self) -> &str {
            &self.value
        }
    }

    struct Consonant<'a> {
        value: &'a str,
    }
    impl Consonant<'_> {
        fn hello(&self) {
            println!(
                "hello!, Im {}. value is {} starting with consonant",
                Consonant::required_process(),
                self.value
            );
            println!("common process is {}", Consonant::common_process());
        }
    }
    //trait implementation for Consonant
    impl Pronunciation for Consonant<'_> {
        fn required_process() -> String {
            "class Consonant".to_string()
        }

        fn common_process() -> String {
            "overriding process".to_string()
        }

        fn value(&self) -> &str {
            &self.value
        }
    }

    //trait as argument
    fn _order(a: &impl Pronunciation, b: &impl Pronunciation) -> bool {
        a.value() < b.value()
    }
    //`impl Trait` statement is syntacs sugar of trait bound `<T: ...Traits>`
    //equal to
    // fn order<T: Pronunciation, U: Pronunciation>(a: &T, b: &U) -> bool {...}
    //can't express below with impl Trait
    fn _order_same<T: Pronunciation>(a: &T, b: &T) -> bool {
        a.value() < b.value()
    }

    //`where` statement make codes more readable
    //if you unuse where,
    //`fn _complex<T: std::cmp::Ord + std::error::Error, U: ..., V: ...>(){}` - awful code
    fn _complex<T, U, V>(_a: T, _b: U, _c: V) -> bool
    where
        T: std::cmp::Ord + std::error::Error,
        U: std::future::Future + std::clone::Clone + Pronunciation,
        V: std::default::Default + std::fmt::Write,
    {
        true
    }

    //trait as returned value
    //it reduce code for returned value
    // telling compiler this fn return some type implementing Copy
    //but the value is limited a type in actual
    //for return multi type impl Copy, need trait object #section 17
    fn _dummy() -> impl Copy {
        3
    }

    let mut s = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    println!("lets see the trait defined function");
    println!("create class `Vowel` or `Consonant`, only enter to quit");
    loop {
        std::io::stdin().read_line(&mut s).unwrap();

        let trimed = s.trim_end();

        match trimed.chars().next() {
            Some(head) => {
                if vowels.contains(&head) {
                    (Vowel { value: trimed }).hello();
                } else {
                    (Consonant { value: trimed }).hello();
                }
            }
            None => {
                println!("empty string, quit");
                break;
            }
        }

        s.clear();
    }

    println!("----------------------------------------");
}

fn demo_lifetime() {
    println!("lifetime!");
    //lifetime guarantees the refference is valid while required,
    //in addition to type properties
    //any value is managed by `owenership`
    //any reference is managed by `lifetime`

    println!("'a, 'b are lifetime annotation for r and x each other");

    println!(
        "
    below code is compile error cuz x's lifetime 'b is too short
    {{
      let r;                  // ---------+-- 'a
                              //          |
      {{                       //          |
        let x = 5;            //  -+-- 'b |
        r = &x;               //   |      |
      }}                       //  -+      |
                              //          |
    println!(\"r: {{}}\", r);     //          |
    }}                         // ---------+    
  "
    );

    println!(
        "
  below can be run
  {{
    let x = 5;                // ----------+-- 'b
                              //           |
    let r = &x;               // --+-- 'a  |
                              //   |       |
    println!(\"r: {{}}\", r);     //   |       |
                              // --+       |
  }}                           // ----------+
  "
    );

    //lifetime annotation - starts wieh `'` and named lower case alphabet
    //  &i32 // a reference
    //  &'a i32 // a reference with an explicit lifetime
    //  &'a mut i32 // a mutable reference with an explicit lifetime
    //this is not affect its refference lifetime directly
    //but informs compiler that such refferences are alive along their lifetime annotation
    // = relations between lifetime

    //lifetime annotation for function
    fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
        //indicates result lifetime is the shorter one of x's or y's
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long"); //          ---------------+- 'a
    {
        //                                                                        |
        let string2 = "xyz"; //                                    -------+- 'b   |
        let result = longest_str(string1.as_str(), string2); //           |       | <- result indicates something valid (here, 'b)
        println!("The longest string is {}", result); //                  |       |
    } //                                                           -------+       |
      //                                                                          |

    //lifetime annotation for structure
    struct _Life<'a> {
        //means structure's instance lifetime is shorter than its field
        part: &'a str,
    }
    //for method
    impl<'a> _Life<'a> {
        fn _level(&self) -> i32 {
            3
        }
    }

    //lifetime elision rule's - see, https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

    //static lifetime -> alive while program run
    // hard code to program's binary - not to use without consideration
    let _stat: &'static str = "static lifetime";
}

fn summary() {
    trait Interface<'a> {
        fn process(&self) -> &'a str;
        fn process_with_default() -> &'a str {
            "default"
        }
    }

    struct Summary<'a, T> {
        s: &'a str,
        i: i32,
        t: T,
    }
    impl<'a, T> Summary<'a, T> {
        fn new(s: &str, i: i32, t: T) -> Summary<T> {
            Summary { s, i, t }
        }

        fn _hello() {
            println!("hello");
        }
    }
    impl<'a, 'b, T> Interface<'a> for Summary<'b, T> {
        fn process(&self) -> &'a str {
            "process"
        }
    }

    fn summary_function<'a, T>(hello: &'a str, sum: T) -> String
    where
        T: Interface<'a>,
    {
        format!("{}, {}", hello, sum.process())
    }

    println!(
        "{}",
        summary_function("hello", Summary::new("sum", 45, 5.0))
    );
}
