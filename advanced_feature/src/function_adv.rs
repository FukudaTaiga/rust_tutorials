pub fn function_adv_demo() {
    function_pointer();
    return_closure();
}

fn function_pointer() {
    //closure can get closure as arg, function too
    println!("function pointer");

    //higher order function

    //closure has a trait `Fn`, it is not a type
    fn arg_closure<T: Fn(i32) -> i32>(c: T) {
        println!("closure executed result: {}", c(1));
    }
    arg_closure(|i| i + 100);

    //any function has a type `fn`
    //fn implements `Fn`, `FnMut`, `FnOnce`
    //so recommended to use generics and closure trait unless there is special reason (like FFI)
    fn arg_function(f: fn(i32) -> i32) {
        println!("function executed result: {}", f(1));
    }
    fn tmp(i: i32) -> i32 {
        i + 100
    }
    arg_function(tmp);
}

fn return_closure() {
    println!("returning closure with trait dyn");
    //create closure
    fn create(value: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |i| i + value)
    }

    let closure = create(100);
    println!("created closure executed result: {}", closure(1));
}
