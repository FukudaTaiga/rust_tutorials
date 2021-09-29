//message sending concurrency - with channel
//  channel have two halves - transmitter and reciever
//  channel closed if its transmitter or receiver dropped
//mpsc ~ multiple producer, single consumer -- actor model. allow multi senders but limit only one receiver
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn channel() {
    usage_channel();
    multiple_msg();
    send_from_multi_thread();
}

fn usage_channel() {
    println!("channel up");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("Msg Send from spawned");
        tx.send(val).unwrap(); //sending value is moved
                               //println!("{}", val); //compile error cuz ownership - if it is allowed, the value main thread may change at receiving
    });

    let received = rx.recv().unwrap(); //blocking the thread exec until msg arrives
                                       //let received = rx.try_recv().unwrap();  //non-blocking the thread exec and returning receivers value immediately
    println!("main got: {}", received);
    println!();
}

fn multiple_msg() {
    println!("multi msg from another thread");
    let (tx, rx) = mpsc::channel();
    let msgs = vec!["I", "am", "from", "main"];

    thread::spawn(move || {
        for msg in msgs {
            tx.send(msg.to_string()).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        //wait msg until receiving
        println!("main got: {}", received);
    }

    //channel will close when spawned tx drop
    println!();
}

fn send_from_multi_thread() {
    println!("send from multi thread");
    let (tx, rx) = mpsc::channel(); //to use same tx among threads, `clone(&tx)`

    let tx1 = mpsc::Sender::clone(&tx);
    let handle1 = thread::spawn(move || {
        let msgs = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the thread"),
            String::from("1"),
        ];
        for msg in msgs {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    let tx2 = mpsc::Sender::clone(&tx);
    let handle2 = thread::spawn(move || {
        let msgs = vec![
            String::from("more messages"),
            String::from("from"),
            String::from("the thread"),
            String::from("2"),
        ];
        for msg in msgs {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    thread::spawn(move || {
        let msgs = vec![
            String::from("if this block"),
            String::from("not exec,"),
            String::from("main wait eternally"),
            String::from("cuz both tx and rx won't drop"),
        ];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("main got {}", received);
    }

    println!();
}
