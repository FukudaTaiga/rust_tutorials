// Section 2
// standard library.
// you can use the types in prelude by default.
// `use ~` to import required type
use rand::Rng;
use std::cmp::Ordering; // enum - Less, Equal, Greater
use std::io; // import trait !!necessary

mod guess {
  pub struct Guess {
    value: i32,
  }

  impl Guess {
    pub const MIN: i32 = 1;
    pub const MAX: i32 = 10000;
  }

  impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
      if value < Guess::MIN || Guess::MAX < value {
        Err(format!(
          "secret value is between {} and {}",
          Guess::MIN,
          Guess::MAX
        ))
      } else {
        Ok(Guess { value })
      }
    }

    pub fn get(&self) -> i32 {
      self.value
    }
  }
}

fn main() {
  use guess::Guess;

  println!("Guessing game!");

  let secret = rand::thread_rng().gen_range(Guess::MIN, Guess::MAX); //generate 1 ~ 100
                                                                     //println!("Secret is {}", secret);
  let mut challenge = 0;

  println!("you have 20 chances to guess");

  loop {
    println!("Please input your guess");

    //let guess_imut = String::new(); <- immutable var
    // mutable var
    // static method `String::new()` of class String
    let mut guess = String::new();

    io::stdin() // <- std::io::Stdin object = handler of stdin
      .read_line(&mut guess) // read_line require mutable reference. return (io::)Result - enum variant, Ok or Err
      .expect("Failed to read"); // expect return if Ok - value, Err - crash and log an argument

    //shadow variable. :u32, type annotation. expect() -> match ... {} is general strategy to handle error.
    let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!(
          "input must be integer between {} {}",
          Guess::MIN,
          Guess::MAX
        );
        continue;
      }
    };

    let guess = match Guess::new(guess) {
      Ok(g) => g,
      Err(msg) => {
        println!("{}", msg);
        continue;
      }
    };

    challenge += 1;

    match guess.get().cmp(&secret) {
      Ordering::Less => println!("your guess is smaller than secret"),
      Ordering::Equal => {
        println!("you win");
        println!("you guessed {} times", challenge); // {} is placeholder of variant like %s in c
        break;
      }
      Ordering::Greater => println!("your guess is bigger than secret"),
    }

    if 20 < challenge {
      println!("you lose");
      println!("the secret is {}", secret);
      break;
    }
  }
}
