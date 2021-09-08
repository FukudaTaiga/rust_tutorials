/**
 * Section 2
 * standard library.
 * you can use the types in prelude by default.
 * `use ~` to import required type
*/
use std::io;
use std::cmp::Ordering; // enum - Less, Equal, Greater
use rand::Rng; // import trait !!necessary


fn main() {
    println!("Guess the number");

    let secret = rand::thread_rng().gen_range(1, 101); //generate 1 ~ 100
    //println!("Secret is {}", secret);

    loop {
      println!("Please input your guess");

      //let guess_imut = String::new(); <- immutable var
      let mut guess = String::new(); // <- mutable var
      //String::new() <- static method of class String

      io::stdin() // <- std::io::Stdin object = handler of stdin
        .read_line(&mut guess) // read_line require mutable reference. return (io::)Result - enum variant, Ok or Err
        .expect("Failed to read"); // expect return if Ok - value, Err - crash and log an argument

      //shadow variable. :u32, type annotation. expect() -> match ... {} is general strategy to handle error.
      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

      println!("You guessed {}", guess); // {} is placeholder of variant like %s in c

      match guess.cmp(&secret) {
        Ordering::Less => println!("small"),
        Ordering::Equal => {
          println!("win");
          break;
        },
        Ordering::Greater => println!("big")
      }
    }
}
