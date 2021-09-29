use std::thread;
use std::time::Duration;

pub fn usage() {
  spawn_thread();
  thread::sleep(Duration::from_millis(12));
  join_thread();
  split_thread_exec();
  capture_env_to_spawned();
}

fn spawn_thread() {
  println!("only spawn");
  thread::spawn(|| {
    for i in 1..10 {
      println!("hi, number {} from the spawned thread", i);
      thread::sleep(Duration::from_millis(1));  //stop this threads excution
    }
  }); //ends if the main thread ends regardless of the spawned is ends

  //so above must not ends
  for i in 1..5 {
    println!("hi, number {} from the main thread", i);
    thread::sleep(Duration::from_millis(1));
  }
  println!("function ends");
}

fn join_thread() {
  println!("joined");
  //JoinHandle is returned
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi, number {} from the spawned thread", i);
      thread::sleep(Duration::from_millis(1));  //stop this threads excution
    }
  }); //ends if the main thread ends regardless of the spawned is ends

  //so above must not ends
  for i in 1..5 {
    println!("hi, number {} from the main thread", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap(); //wait for the thread - blocking
  println!("function ends");
}

fn split_thread_exec() {
  println!("split");
  //JoinHandle is returned
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi, number {} from the spawned thread", i);
      thread::sleep(Duration::from_millis(1));  //stop this threads excution
    }
  }); //ends if the main thread ends regardless of the spawned is ends

  handle.join().unwrap(); //this will split execution - timing of blocking is necessary

  //so above must not ends
  for i in 1..5 {
    println!("hi, number {} from the main thread", i);
    thread::sleep(Duration::from_millis(1));
  }

  println!("function ends");
}

fn capture_env_to_spawned() {
  println!("capture envs");
  let v = vec!["I", "am", "from", "main",];

  let handle = thread::spawn(move || { //take away v's ownership from main
    println!("{}", v.join(" "));
  });

  //drop(v);  //compile error cuz v was already moved

  handle.join().unwrap();
}