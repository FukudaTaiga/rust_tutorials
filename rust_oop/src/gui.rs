#![allow(dead_code)]
//trait object is a functionality to use trait as if concrete type
//it indicates some type's pointer like &ref or smart pointer
//    implementing the trait, which can't be known when coding

pub trait Draw {
  fn draw(&self);
}

//implements Screen for all type satisfying the requirement
//  trait object is recommended to use with the syntax `dyn` -- RFC2113
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>, //trait object - can set [Button, TextField, ...etc]
}
impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}
//implements Screen for some type T satisfying the requirement
//  monomorphization when code compiled
pub struct ScreenWithG<T: Draw> {
  pub components: Vec<T>, //generics - can't set [Button, TextField, ...etc]
}
impl<T: Draw> ScreenWithG<T> {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button<'a> {
  pub width: u32,
  pub height: u32,
  pub label: &'a str
}
impl Draw for Button<'_> {
  fn draw(&self) {
    if self.width < self.label.len() as u32 + 2 {
      return ()
    }

    for i in 0..self.height + 2 {
      let output = if i == 0 || i == self.height + 1 {
        "-".repeat(self.width as usize)
      }
      else if i == self.height / 2 + 1 {
        let space = (self.width - 2 - self.label.len() as u32) / 2;
        format!("{}{}{}{}{}", "|", " ".repeat(space as usize), self.label, " ".repeat(space as usize), "|")
      }
      else {
        format!("{}{}{}", "|", " ".repeat((self.width - 2) as usize), "|")
      };

      println!("{}", output);
    }
  }
}

pub struct SelectBox {
  pub width: u32,
  pub height: u32,
  pub options: Vec<String>
}
impl Draw for SelectBox {
  fn draw(&self) {
    if self.options.len() == 0 {

      return ()
    }

    println!("{}", "-".repeat(self.width as usize));
    for i in 0..self.options.len() {
      println!("{}", &self.options[i])
    }
    println!("{}", "-".repeat(self.width as usize));
  }
}