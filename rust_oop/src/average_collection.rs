//AverageCollection provides dealing with vector,
//but such operation is not applied directly, through API
//  these encapsulation is one of the feature of oop

#[derive(Debug)]
pub struct AverageCollection {
  list: Vec<i32>,
  average: f64,
}
impl AverageCollection {
  pub fn new(init: Vec<i32>) -> AverageCollection {
    let mut ave_coll = AverageCollection {
      list: init,
      average: 0.0,
    };
    ave_coll.update_average();
    println!("Average Collection created,\n{:#?}", ave_coll);
    ave_coll
  }

  pub fn add(&mut self, item: i32) {
    self.list.push(item);
    self.update_average()
  }

  pub fn remove_last(&mut self) -> Option<i32> {
    let last = self.list.pop();
    match last {
      Some(value) => {
        self.update_average();
        Some(value)
      }
      None => None,
    }
  }

  fn update_average(&mut self) {
    if self.list.len() == 0 {
      println!("warning!: list is empty");
      self.average = 0.0;
    } else {
      self.average = self.list.iter().sum::<i32>() as f64 / self.list.len() as f64
    }
  }
}
