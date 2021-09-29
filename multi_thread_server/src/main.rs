//sec 20

//1. learn TCP, HTTP
//2. listen TCP with socket
//3. parse HTTP request structure 
//4. response HTTP
//5. emphasis server's throughput with thread pool
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    //connect TCP connection on 127.0.0.1:7878
    println!("Server Successfully started. see http://localhost:7878");

    for stream in listner.incoming() {
      let stream = stream.unwrap();

      println!("Connection established");
      handle_connection(stream);
    }
}

//TcpStream trace the data internally, so stream should be mutable even if read only
fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024]; //its enough in most cases
  let mut contents = String::new();

  stream.read(&mut buffer).unwrap();  //read from some stream
  
  let root = b"GET / HTTP/1.1\r\n";
  let js = b"GET /addtext.js HTTP/1.1\r\n";
  let css = b"GET /index.css HTTP/1.1\r\n";
  
  let (status, filename) = if buffer.starts_with(root) {
    (200, "html/index.html")
  }
  else if buffer.starts_with(js) {
    (200, "html/addtext.js")
  }
  else if buffer.starts_with(css) {
    (200, "html/index.css")
  }
  else {
    (404, "Invalid")
  };

  if status == 200 {
    File::open(filename).unwrap().read_to_string(&mut contents).unwrap();
  }
  else {
    contents.push_str("Invalid");
  }
  let response = format!("HTTP/1.1 {} OK\r\n\r\n{}", status, contents);

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
  //wait to flush until ensuring the operation finished

  println!("Request: {}", String::from_utf8_lossy(&buffer));
  //lossy define the behavior when see an invalid sequense - replace it with �, U+FFFD REPLACEMENT CHARACTER
}
