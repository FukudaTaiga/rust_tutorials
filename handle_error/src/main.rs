//two type error
// recoverable error ... can't find some file
//    - handle by Result enum
// unrecoverable error ... access array item out of index
//    - handle by panic macro

fn main() {
  //generally, panic is prefferable while development - when providing library or special case
  //Result propagate error to caller

  //_unrecoverable_error();
  recoverable_error();
}

fn _unrecoverable_error() {
  //panic macro
  //if panic happens, then there are two approach considerable
  //  One, unwinding and clean up data - this work take hard
  //  Alternative, immidiately aborting - such work done by OS
  // default is the former, if needed in release then add
  //   [profile.release]
  //    panic = 'abort'

  //panic!("crash and burn"); //crash

  //back trace
  let v = vec![1, 2, 3];
  v[99]; //runtime error
         // to get more information, `RUST_BACKTRACE=1 cargo run`
}

fn recoverable_error() {
  use std::fs::{File, OpenOptions};
  use std::io::ErrorKind;
  use std::io::{Read, Write};
  //enum Result<T, E> {
  //  Ok(T), - success
  //  Err(E), - failue
  //}
  const DUMMY: &str = "dummy.txt";

  let f = OpenOptions::new().write(true).truncate(true).open(DUMMY);
  //to check funcion returning value, read doc
  //or ask compiler - intend to annotate the type not corresponding to

  let mut f = match f {
    Ok(file) => file,
    Err(ref e) if e.kind() == ErrorKind::NotFound => match File::create(DUMMY) {
      //ref is used for guard e value moving
      Ok(fc) => fc,
      Err(e) => {
        panic!("Filed to create File, {:?}", e);
      }
    },
    Err(e) => {
      panic!("Filed to open File {:?}", e);
    }
  };

  match f.write_all("hello world".as_bytes()) {
    Ok(_) => (),
    Err(e) => {
      panic!("Failed to write: {:?}", e);
    }
  }

  //handling error shortcut - unwrap, expect, ...
  //`match` is very strong way to handle pattern, but it can be verbose
  //some of methods on Result like `unwrap()` and `expect()` are helpful 
  let _f = File::open(DUMMY).unwrap(); //equal to below
                                       // match File::open(DUMMY) {
                                       //     Ok(value) => value,
                                       //     Err(e) => {
                                       //        panic!("{:?}", e);
                                       //     }
                                       // }
  let _f = File::open(DUMMY).expect("some message for an error"); //equal to below
                                                                  // match File::open(DUMMY) {
                                                                  //     Ok(value) => value,
                                                                  //     Err(e) => {
                                                                  //        panic!("some message for an error: {:?}", e);
                                                                  //     }
                                                                  // }

  //propagating error - ? operator
  //? operator can only be used in the function returning Result<T, E>
  fn propagate_error_sample(name: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    //these code is equal to following
    //let mut f = match File::open(name) {
    //  Ok(file) => file,
    //  Err(e) => return Err(e)
    //};
    //let mut s = String::new();
    //match f.read_to_string(&mut s) {
    //  Ok(_) => Ok(s),
    //  Err(e) => Err(e)
    //}
  }

  let s = propagate_error_sample(DUMMY).unwrap();
  println!("{}", s);

  //Section 9.3 `when should we panic?` is very helpful
}
