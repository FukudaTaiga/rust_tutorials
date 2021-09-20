//Sec 16
//OOP - introduced by Alan Kay, at first
//Object-oriented programs are made up of objects.
//An object packages both data and the procedures that operate on that data.
//The procedures are typically called methods or operations.
mod average_collection;
mod gui;
use average_collection::AverageCollection;
use gui::*;

fn main() {
  capsule_ex();
  type_system_inheritance();
  state_pattern();
}

fn capsule_ex() {
  //if codes are encapsuled properly, code changing locally becomes easy
  //  managing public or private

  println!("encapsulation!");
  let mut a1 = AverageCollection::new(vec![]);
  let _a2 = AverageCollection::new(vec![1, 2, 3, 4, 5]);

  a1.add(100);
  a1.add(200);
  a1.add(300);
  println!("a1:\n{:#?}", a1);
  println!("remove a1's last: {:?}", a1.remove_last());
  println!("a1:\n{:#?}", a1);

  println!("----------------------------------------");
}

fn type_system_inheritance() {
  //if inheritance is needed, Rust is not OOP
  //but Rust provides the functionality to implement the motivation to use inheritance
  //there is two general use case --
  //first:  to reuse code -- with default implementation of trait
  //second: polymorphism -- with trait object

  println!("type system and inheritance");
  //trait object is similar to `duck typing` in dynamically typed lang
  //typing from its behavier
  //  compile assuming the type has certain fields and methods
  //  and check the existance when running
  //    -> but trait object check such existance, not to be afraid of runtime error
  //however, trait object is some kind of pointer whose type implements a certain trait

  let button1 = Box::new(Button {
    width: 5,
    height: 1,
    label: "btc",
  });
  let button2 = Box::new(Button {
    width: 5,
    height: 3,
    label: "eth",
  });
  let sbox = Box::new(SelectBox {
    width: 10,
    height: 0,
    options: vec![
      String::from("option1"),
      String::from("option2"),
      String::from("option3"),
    ],
  });

  let screen = Screen {
    components: vec![button1, sbox, button2],
  };
  //compile error - cuz monomorphization
  //let screen_with_g = ScreenWithG {
  //  components: vec![button1, sbox, button2]
  //};

  screen.run();

  println!(
    "
  trait object's performance and requirement:
    trait object takes cost of dynamic dispatch - unlike static dispatch
    compiler can't get the actual method to called
    it determines the method in runtime
  "
  );

  println!(
    "
  trait object requests object safety:
  - many requirements needed, but the two rules are related inactual
  1. return value is not Self
  2. no Generic type argument
  these are for dynamic dispatch, duck typing
  "
  );

  //Self is the abbrevigation of its type
  struct _A {}
  impl _A {
    fn _a() -> Self {
      //Self is A, here
      _A {}
    }
  }

  println!("----------------------------------------");
}

fn state_pattern() {
  println!("state pattern!");

  println!("TODO");

  println!("----------------------------------------");
}
