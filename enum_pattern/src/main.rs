// Sec 6
// Rust enum very resembles to Functional PL like Haskell, Ocamel, F#
// Option<T> {Some(T), None} is prelude compiled - no import to use

use rand::Rng;

fn main() {
  enum_basic();
}

fn enum_basic() {
  enum _Basic {
    A,
    B,
    C,
  }
  enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
  }

  let _loopback = IpAddr::V4(127, 0, 0, 1);
  let _loopback = IpAddr::V6(String::from("::1"));

  #[derive(Debug)]
  enum Message {
    Quit,
    Move { x: f64, y: f64 },
    Write(String),
    Color(i32, i32, i32),
  }
  impl Message {
    // method definition
    fn print(&self) {
      //pattern match for exact one or the others
      if let Message::Quit = self {
        println!("message: {:?} !", self);
      } else {
        println!("message: {:?}", self);
      }
    }
    fn quit() -> Message {
      Message::Quit
    }
    fn write(value: String) -> Message {
      Message::Write(value)
    }
    fn mv(x: f64, y: f64) -> Message {
      Message::Move { x, y }
    }
    fn color(r: i32, g: i32, b: i32) -> Message {
      Message::Color(r, g, b)
    }
  }

  #[derive(Debug)]
  struct Color(i32, i32, i32);
  #[derive(Debug)]
  struct State {
    x: f64,
    y: f64,
    contents: String,
    color: Color,
  }
  impl State {
    fn action(&mut self, op: Option<Message>) {
      match op {
        // pattern match
        Some(msg) => {
          msg.print();
          match msg {
            Message::Quit => return (),
            Message::Move { x, y } => {
              self.x += x;
              self.y += y;
            }
            Message::Write(str) => self.contents.push_str(&str[..]),
            Message::Color(r, g, b) => {
              self.color.0 += r;
              self.color.1 += g;
              self.color.2 += b;
            }
            _ => (), //wild card
          }
        }
        None => println!("done nothing"),
      }
    }
    fn print(&self) {
      println!("current: {:#?}", self)
    }
  }
  const INITIAL: State = State {
    x: 0.0,
    y: 0.0,
    contents: String::new(),
    color: Color(0, 0, 0),
  };

  let mut state = INITIAL;
  let mut counter = [0,0,0,0];
  loop {
    let random = rand::thread_rng().gen_range(0, 1000);
    state.action(if random == 0 {
      Some(Message::quit())
    } else if random % 4 == 0 {
      counter[0] += 1;
      Some(Message::write(String::from("a")))
    } else if random % 4 == 1 {
      counter[1] += 1;
      Some(Message::mv(1.5, 0.5))
    } else if random % 4 == 2 {
      counter[2] += 1;
      Some(Message::color(1, 2, 3))
    } else {
      counter[3] += 1;
      None
    });
    if random == 0 {
      state.print();
      println!("w: {}", counter[0]);
      println!("m: {}", counter[1]);
      println!("c: {}", counter[2]);
      println!("n: {}", counter[3]);
      break;
    }
  }
}
