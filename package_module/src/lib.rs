//rust project has 0 or 1 library crate and more than 1 binary crate
//module handle privacy: is implementation public or private

use std::{cmp::Ordering, io}; //nesting import
//equal to `use std::cmp::Ordering; use std::io;`
use std::time::{self, SystemTime};
//equal to `use std::time; use std::time::SystemTime;`
use std::vec::*; //glob operator
pub mod other; //import other module with other.rs file
mod other2; //import other2 module with other2/mod.rs
//even if not declared mod ~~~, dividing file make module named as filename
//example: create exam.rs, then `mod exam;` imports the contents of it (if directry, imported file become mod.rs under it)

mod front_of_house {
  //nesting
  pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}
    fn take_payment() {}
  }
}

pub use crate::front_of_house::hosting; //make external file to use hosting like exported hosting as root

fn serve_order() {
  use crate::front_of_house::hosting; // import for shorthand
  //equal to `use self::front_of_house::hosting`
  hosting::add_to_waitlist(); //available as if hosting defined in the same root

  //snippet
  use crate::back_of_house::Breakfast as Abrevigation;
  let _abbre = Abrevigation::summer("Rye");
}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order(); //same root
    super::serve_order(); // super indicates module `crate::front_of_house::serving`
  }
  fn cook_order() {}

  // manage privacy each field
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }
  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }

  //every component become public if enum checked as `pub`
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

pub fn eat_at_rastaurant() {
  //call with Absolute path - starts with `crate` or crate name
  crate::front_of_house::hosting::add_to_waitlist();
  //call with Relative path - self, super, or identifier in the module
  // front_of_house is in the same level as this fuction, so this cause no error
  front_of_house::hosting::add_to_waitlist();

  //front_of_house::serving::~ compile error cuz serving is private
  //front_of_house::hosting::seat_at_table() compile error too

  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast, please", meal.toast);
  //meal.seasonal_fruit = String::from("blueberries"); compile error
  //meal.seasonal_fruit is private
}

fn main() {
  eat_at_rastaurant();
  other::other_print();
  other2::other2_child::other2_child_print()
}
