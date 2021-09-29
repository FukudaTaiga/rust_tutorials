//shared state (memory) is another way to deal with concurrency - lock system
//Mutex(mutual exclusion) allows only one thread to access data at the same time
//Mutex require two rules
//  1. acquire the lock the data before use
//  2. unlock it after use
//sometimes Mutex controll become incredibly tricky
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn mutex() {
    usage_mutex();
    thread_mutex();
    dead_lock_avoidance();
}

fn usage_mutex() {
    println!("simple usage");
    //Mutex is smart pointer
    let m = Mutex::new(5);

    println!("m = {:?}", m);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

/*
  analogies between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>
    RefCell/Mutex provides internal mutability

    Rc/RefCell have a risk of cyclic reference/dead lock
  Mutex won't help to avoid such logic error
*/
fn thread_mutex() {
    println!("thread mutex and arc");
    let counter = Arc::new(Mutex::new(0)); //Rc is only for single thread, Arc - atomic ref counter needed
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            //there happens value moving, so Refference counter needed
            let mut num = counter.lock().unwrap();
            println!("hello, Im {}th spawned thread, counter is {}", i, num);
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "hello, Im main thread, counter is {}",
        counter.lock().unwrap()
    )
}

fn dead_lock_avoidance() {
    println!(
        "
  TODO --
  make a program that have dead lock and 
  research its mitigation strategy, implement it
  "
    );
}
