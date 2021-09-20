//Sec 15
#![feature(box_syntax)]
//pointer indicates address on the memory
//  basic example is refference (like `&x`)
//smart pointer is a data structure originated from c++
//  which behaves as pointer and has additional meta data and ability
//one of the major difference is
//  pointer borrows its value but
//  smart pointer owns its value
//smart pointer is implemented as structure that extends traits Deref and Drop
//  Deref makes it behave as if pointer
//  Drop provides the behavier of droppig its value (`drop` is called when it goes out of scope)

use std::cell::RefCell;
use std::ops::{Deref, Drop}; //no need to import Drop for the trait needed in initializing
use std::rc::{Rc, Weak};

fn main() {
  //Summary of the recommended choice
  //  RC - to manage multiple ownership, only for immutable borrowing
  //  Box - to force the data on heap, for immutable/mutable borrowing (single thread)
  //  RefCell - to use internal mutability, for irregular pattern borrowing (single thread)

  sp_box();
  sp_rc();
  sp_ref_cell();
  sp_rc_weak();
}

fn sp_box() {
  println!("smart pointer Box!");
  //Box<T> trait forces to store the value on a heap instead of stack
  //Box provides heap memory and an undirected refferrence

  //usage
  {
    let b = Box::new(5);
    println!("the value is stored in a heap: {}", b);

    let unstable = box 5; //need to run `cargo +nightly run`
    println!("unstable {}", unstable);

    //difference between aboves
    //`Box::new()` create data on stack once and move it to heap -> it could cause stack overflow
    //`box` syntax put it into heap directly --- its not so simple
  }

  //recursive type - can't determine its size for such type in compile, but can for the Box
  //Cons List - originated from Lisp lang
  enum List<T> {
    //enum size is the biggest one of its member
    Cons(T, Box<List<T>>),
    Nil,
  }
  println!(
    "
the List's memory representation
-without Box, can't determine its size
  |   |   List(Cons)      |
  |i32|   |   List(Cons)  |
  |   |i32|   | List(Cons)|
  |   |   |i32|     :     |
  |   |   |   |     :     |
-with Box, can
  List(Cons)      |   List(Cons)      |   List(Nil)      |
  |i32|   Box   | |   |i32|   Box   | |   |i32|   Box  | |
  |   |  |usize-+-+-->|   |  |usize-+-+-->|   |  |usize| |

so List need memory size 
  List(Cons)     |
  |i32|   Box  | |
  |   |  |usize| |
  "
  );

  //derefferrence operator
  {
    let x = 5;
    let y = &x;
    println!("original value: {}", x);
    println!("borrowed value: {}", *y);
    //assert_eq!(y, 5); can't compare `&{integer}` with `{integer}`
    assert_eq!(*y, 5);

    let z = Box::new(x);
    println!("smart pointer Box's value: {}", *z); //work cuz Box extends Deref trait
    assert_eq!(*y, 5);
  }

  //original smart pointer with Deref
  {
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
      fn new(value: T) -> MyBox<T> {
        MyBox(value)
      }
    }
    impl<T> Deref for MyBox<T> {
      //there is a trait DerefMut to deref of mutable ref
      type Target = T;

      //whenever dereferenced, its value must not be steeled, so return its value's pointer
      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }

    let x = 5;
    let y = MyBox::new(x);
    //Rust automatically translated * to *(.deref()) if needed
    //this translation will be done "Only Once" every * use
    assert_eq!(*y, *(y.deref()));

    //(implicit) force of dereference - Rust feature to help of coding
    //this architechture seems not to be known so much
    //it will be added if I feel it needed to know
  }

  //original smart pointer with Drop
  {
    struct CustomData {
      data: String,
    }
    impl CustomData {
      fn new(data: &str) -> CustomData {
        println!("CustomData created with the value `{}`", data);
        CustomData {
          data: data.to_string(),
        }
      }
    }
    impl Drop for CustomData {
      fn drop(&mut self) {
        println!("the CustomData is dropping! its data is `{}`", self.data);
      }
    }

    let first = CustomData::new("first");
    let second = CustomData::new("second");
    let third = CustomData::new("third");

    //second.drop(); forbidden to use destructor explicitly
    //instead, use `std::mem::drop()`
    drop(second); //explicitly use of drop()
  }

  println!("----------------------------------------");
}

fn sp_rc() {
  println!("referrence counting!(RC)");
  //RC represents multiple ownership
  //(with single thread!! in concurrent processing, unable to use)

  //consider to express Lists like below
  //  b - [3]
  //         \
  //      a - [5] - [10]    this can't be express by the List with Box cuz Box's smart pointer moving the value
  //         /
  //  c - [4]

  //RC manages how many owner uses the value
  //its value never drop until no owner use it
  //RC has a counter for it
  #[derive(Debug)]
  enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
  }
  {
    use List::{Cons, Nil};
    //get reference - with Rc::clone or &self.clone
    //Rc::clone(&self) - only increment the counter, low cost
    //&self.clone() - deep copy, expensive in some case
    //  Rc::strong_count(&self) return current count of the ref

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("List a created");
    println!("{:?}", a);
    println!("a's RC: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("List b created");
    println!("{:?}", b);
    println!("a's RC: {}", Rc::strong_count(&a));
    {
      let c = Cons(4, Rc::clone(&a));
      println!("List c created");
      println!("{:?}", c);
      println!("a's RC: {}", Rc::strong_count(&a));
    }
    println!("List c goes out of scope");
    println!("a's RC: {}", Rc::strong_count(&a));
  }

  println!("----------------------------------------");
}

fn sp_ref_cell() {
  //trait RefCell - enable to change the value even in immutable reference (internal mutability)
  //(for single thread like RC, not for concurrency)
  //  borrowing rule requests,
  //    - only one mutable reference or,
  //    - one or more immutable reference
  //  for some value at the same time
  //RC provides immutable changable reference and passing compiler
  //RefCell is usefull in such situation, but of course panic in runtime
  //  like a MockObjects, tests ---- skip
  // general use case is multiple mutable reference with RC + RefCell
  println!("RefCell! (+ RC)");

  #[derive(Debug)]
  enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
  }

  //hidden mutable reference
  {
    let ref_value = Rc::new(RefCell::new(5));
    let a = Rc::new(List::Cons(Rc::clone(&ref_value), Rc::new(List::Nil)));
    let a_clone = a.clone();
    let b = List::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));
    println!("a: {:?}", a);
    println!("a's clone: {:?}", a_clone);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("current count of a: {}", Rc::strong_count(&a));
    println!(
      "current count of ref_value: {}",
      Rc::strong_count(&ref_value)
    );

    //borrow_mut() return RefMut<T> smart pointer, after deref, getting mutable T
    println!("add 10 to ref_value");
    *ref_value.borrow_mut() += 10;
    println!("a: {:?}", a);
    println!("a's clone: {:?}", a_clone);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("current count of a: {}", Rc::strong_count(&a));
    println!(
      "current count of ref_value: {}",
      Rc::strong_count(&ref_value)
    );

    println!("add 10 to ref_value");
    *ref_value.borrow_mut() += 10;
    println!("a: {:?}", a);
    println!("a's clone: {:?}", a_clone);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("current count of a: {}", Rc::strong_count(&a));
    println!(
      "current count of ref_value: {}",
      Rc::strong_count(&ref_value)
    );
  }

  //reference cycle and memory leak
  {
    println!("check reference cycling");
    #[derive(Debug)]
    enum List<T> {
      Cons(T, RefCell<Rc<List<T>>>),
      Nil,
    }
    impl<T> List<T> {
      fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match *self {
          List::Cons(_, ref item) => Some(item),
          List::Nil => None,
        }
      }
    }

    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    println!(
      "
a
|
5 -> Nil
    "
    );

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    println!(
      "
b     a
|     |
10 -> 5 -> Nil
    "
    );
    if let Some(link) = a.tail() {
      *link.borrow_mut() = Rc::clone(&b);
    }
    println!("a's tail set to b");
    println!(
      "
b     a
|     |
10 -> 5
|_____|
    "
    );
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    //stack overflow - if not run, won't go panic, but the b's value won't
    //println!("{:?}", a);
    //compiler can't recognize such sort of bug
    //this is caused by Rc's drop logic --- drop its value if the strong count become zero
  }

  //to avoid it, use Weak<T>, weak count --- its not affect the judge of drop, but able to get the multiple refs

  println!("----------------------------------------");
}

fn sp_rc_weak() {
  println!("RC + RefCell + Weak");

  #[derive(Debug)]
  struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
  }

  //create Weak::new()
  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });
  println!("leaf created\n{:#?}", leaf);

  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf),
  );

  //upgrade - get current inner value, with Option
  println!("leaf's parent: {:?}", leaf.parent.borrow().upgrade()); //will None

  let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
  });
  println!("branch created\n{:#?}", branch);

  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf),
  );

  println!(
    "branch strong = {}, weak = {}",
    Rc::strong_count(&branch),
    Rc::weak_count(&branch),
  );

  //Rc::downgrade(&ref) - return Weak ref of ref,
  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
  //not ending with stack overflow, cuz weak ref
  println!("leaf's parent: {:?}", leaf.parent.borrow().upgrade()); //will Some(branch)

  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf),
  );

  println!(
    "branch strong = {}, weak = {}",
    Rc::strong_count(&branch),
    Rc::weak_count(&branch),
  );

  println!("----------------------------------------");
}
