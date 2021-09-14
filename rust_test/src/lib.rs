pub fn double(x: i32) -> i32 {
    x * x
}

//to unit test, create module `tests` and add annotation `#[cfg(test)]`

//test runs cocurrently by default
//this means if any function with side effect, it will fail though they are correct
//to avoid this, `cargo test -- --test-threads=1`

//to see stdout even if test pass, `cargo test -- --nocapture`

//to filter tests, `cargo test (part of test name)` = execution of the tests which name includes (part of test name)

//PartialEq required when asserting == and !=
//Debug required when printing
//these can be derived, add annotation #[derive(Debug, PartialEq)]
trait Sharp {
  fn area(&self) -> f64;
  fn can_hold(&self, other: &Sharp) -> bool {
    self.area() > other.area()
  }
}

#[derive(Debug, PartialEq)]
struct Rectangle {
  width: i32,
  height: i32,
}
impl Sharp for Rectangle {
  fn area(&self) -> f64 {
    (self.height * self.width) as f64
  }
}
impl Rectangle {
  fn new(width: i32, height: i32) -> Result<Rectangle, String> {
    if width < 0 {
      Err("Invalid width".to_string())
    } else if height < 0 {
      Err("Invalid height".to_string())
    } else {
      Ok(Rectangle { width, height })
    }
  }
}

#[derive(Debug, PartialEq)]
struct Circle {
  radius: i32,
}
impl Sharp for Circle {
  fn area(&self) -> f64 {
    (self.radius * self.radius) as f64 * std::f64::consts::PI
  }
}
impl Circle {
  fn new(radius: i32) -> Circle {
    if radius < 0 {
      panic!("Invalid radius: negative value");
    }

    Circle { radius }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  //ignore
  // to execute only ignored tests, `cargo test -- --ignored`
  #[test]
  #[ignore]
  fn ignored() {
      panic!("ignored test!");
  }

  #[test]
  fn assert_new() {
    let rec = Rectangle::new(5, 12).unwrap();
    let circ = Circle::new(1);

    assert_eq!(
      Rectangle {
        width: 5,
        height: 12
      },
      rec
    );
    assert_eq!(Circle { radius: 1 }, circ);
  }

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle::new(4, 5).unwrap();
    let mid = Rectangle::new(4, 4).unwrap();
    let smaller = Circle::new(1);

    //assert! macro - pass if parameter is true
    assert!(larger.can_hold(&mid));
    assert!(!smaller.can_hold(&mid));

    //assert_eq! macro
    assert_eq!(larger.area(), 20.0);
    assert_ne!(mid.area(), 20.0);
  }

  #[test]
  fn custom_message() {
    let circ = Circle::new(1);
    let result = circ.area();
    //custom message for failure
    assert!(
      result == std::f64::consts::PI,
      "Circle's area is wrong, the value is {}",
      result
    );
  }

  //panic test
  #[test]
  #[should_panic(expected = "Invalid radius: negative value")] //if no expectation, any panic pass this test
  fn circle_negative_radius() {
    Circle::new(-1);
  }

  //with Result
  #[test]
  fn rectangle_negative_value() -> Result<(), String> {
      match Rectangle::new(-1, 1) {
        Ok(_) => Err(String::from("Rectangle::new doesn't catch the error")),
        Err(_) => Ok(()),
      }
  }
}
