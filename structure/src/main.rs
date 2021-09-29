#![allow(dead_code)]
// Sec 5 structure
// struct -> create new type

fn main() {
    normal_structure();
    tuple_structure();
    unit_like_structure();
    method_and_debug();
}

fn normal_structure() {
    struct User {
        username: String, //to set &str, need to define lifetime specifier
        email: String,
        sign_in_count: u64,
        is_active: bool,
    }
    const EMPTY_USER: User = User {
        username: String::new(),
        email: String::new(),
        sign_in_count: 0,
        is_active: false,
    };
    fn _user_builder(username: String, email: String) -> User {
        User {
            username: username, // its tedious
            email,              //shorthand
            ..EMPTY_USER        //auto create missing fields with given structure
                                // is_active: EMPTY_USER.is_active,
                                // sign_in_count: EMPTY_USER.sign_in_count
        }
    }

    let _empty = EMPTY_USER;
    let im_user = User {
        email: String::from("xyz@example.com"),
        is_active: true,
        username: String::from("abc"),
        sign_in_count: 1,
    };
    println!("im_user sign in count: {}", im_user.sign_in_count);
    //im_user.is_active = true; compile error

    let mut mut_user = User {
        email: String::from("uvw@example.com"),
        is_active: false,
        username: String::from("def"),
        sign_in_count: 5,
    };
    println!("before mutate: {}", mut_user.is_active);
    mut_user.is_active = true; //need to mut whole instance
    println!("after mutate: {}", mut_user.is_active);
}

fn tuple_structure() {
    //fields have no name but only type
    struct Color(i32, i32, i32);
    struct Position(i32, i32, i32);

    let black = Color(0, 0, 0);
    let center = Position(0, 0, 0);

    println!("black, 0: {}", black.0);
    println!("center, y: {}", center.1);
}

fn unit_like_structure() {
    //no field
    struct _Unit;
}

fn method_and_debug() {
    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }
    impl Rectangle {
        //method definition
        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn compare(&self, other: &Rectangle) -> bool {
            self.area() < other.area()
        }

        //mut self
        fn update_width(&mut self, width: f64) {
            self.width = width;
        }

        //static method - doesn't argument self
        fn square(edge: f64) -> Rectangle {
            //related function - called by (STRUCT NAME)::(function)
            Rectangle {
                width: edge,
                height: edge,
            }
        }
        // why call doesnt need self-> instead of self. ?
        // Rust get method call, then it check self signature
        // if necessary, auto add * or &
        // below codes are the same
        // rec.compare(&oth) -> (&rec).compare(&oth)
        // rec._take_owner() <- (&rec)._take_owner()
    }
    let mut rec = Rectangle {
        width: 5.0,
        height: 6.5,
    };
    fn _area(rec: &Rectangle) -> f64 {
        rec.width * rec.height
    }
    println!("rec: {:?}", rec); // {:?} enables to convert struct to Display
    rec.update_width(19.0); // if missing `mut` keyword, compile error
    println!("mutate rec by update_width(19.0)");
    println!("rec more {:#?} -- more readable", rec);

    let sq = Rectangle::square(10.0);
    println!("sq: {:#?}", sq);
    println!("sq < rec ? {}", sq.compare(&rec));
}
