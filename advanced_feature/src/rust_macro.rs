pub fn macro_demo() {
  //macro doc https://veykril.github.io/tlborm/
  declaretive();
  procedual();
}

//declaretive macro
//  macro definition is like match statement
//  but it tests the Rust code structure
//  valid macro syntax is https://doc.rust-lang.org/reference/macros-by-example.html
#[macro_export]
macro_rules! myvec {
  ( $( $x: expr ), * ) => 
  // $x: expr - matches any Rust expression and capture it
  // $(  ) - capture replace target block for $(  )
  // , - separator , optionally appears after the matches
  // * - like regex means
  {
    {
      let mut temp_vec = Vec::new();
      $(
          temp_vec.push($x);
      )*  //this $()* code block generated for each match
      temp_vec
    }
  };
}

fn declaretive() {
  let _ = myvec![1, 2, 3];
  println!("Note: Rust prepare to add other way of declaretive macro, then macro_rules! will be deprecated");
}

//procedual macro generating code from attriburtes
//derive macro, attributes like, function like
fn procedual() {
  println!("TODO
--very complex, it should probably excercise with other repo for it
--procedual macros seem to be usefull when make crate\
");
}