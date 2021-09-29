#![allow(dead_code)]

pub fn type_adv_demo() {
    type_alias();
    never_type();
    dynamic_sized();
}

fn type_alias() {
    //type alias provide some types abbrevigation
    //type alias is usefull in two way
    //  1 - writability
    //  2 - consistent interface

    type Integer = i32;
    type Complex = std::cell::RefCell<std::rc::Rc<dyn Fn() + Send + 'static>>;
    println!("keep in mind that type alias is only a type");
}

fn never_type() {
    //never-type is represented by `!`
    //it means Empty type, no value
    //  `continue`, `panic!()`, `loop { no break statement }` is never type

    //function returning ! is called diverging function
    fn bar() -> ! {
        loop {}
    }
    println!("never type is interpret as any type");
}

fn dynamic_sized() {
    //dinamically sized type - DST
    //str is one of DST
    //  str have any length, so the size is not determined in compile
    //  so we use &str - (address, length) = (usize, usize)
    //every trait is also DST
    //  having seen as dyn,
    //to work with DST, there is a trait `Sized`
    //  this trait is automatically implemented for everything whose size is known at compile time
    //    and implicitly add trait bound Sized for any generics

    fn g<T>(_: T) {}
    fn g_actuall<T: Sized>(_: T) {}

    //to avoid this, add bound with special suntax `?Sized`
    //it tells "type may or may not be Sized"
    fn g_avoid<T: ?Sized>(_: &T) {}
    //fn g_avoid<T: ?Sized>(_: T) {} results in error cuz type T can not be Sized
    println!("enjoy DST with correct knowledge");
}
