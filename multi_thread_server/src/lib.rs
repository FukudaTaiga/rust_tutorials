pub mod thread_pool;

use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use thread_pool::ThreadPool;

const URI: &str = "127.0.0.1:7878";

//thread pool - one way to improve throughput, research fork/join model, non-blocking single thread I/O model
//  store fixed many idle threads and adopt one of them to do required process
//take care not to create infinitely many thread

//Compile Driven Development(CDD)
//cycle to write code calling required function, compile and resolve the error

pub fn run() {
    let listner = TcpListener::bind(URI).unwrap();
    let pool = ThreadPool::new(4);

    println!("Multi thread server Successfully started. see http://localhost:7878");

    for stream in listner.incoming()
    //.take(10) // for check server successfully shutting down
    {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        });
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024]; //its enough in most cases
        let mut contents = String::new();

        stream.read(&mut buffer).unwrap(); //read from some stream

        let root = b"GET / HTTP/1.1\r\n";
        let js = b"GET /index.js HTTP/1.1\r\n";
        let css = b"GET /index.css HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

        let (status, filename) = if buffer.starts_with(root) {
            (200, "html/index.html")
        } else if buffer.starts_with(js) {
            (200, "html/index.js")
        } else if buffer.starts_with(css) {
            (200, "html/index.css")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_millis(5000));
            (200, "Im sleeping now")
        } else {
            (404, "Invalid")
        };

        if filename.contains(".") {
            File::open(filename)
                .unwrap()
                .read_to_string(&mut contents)
                .unwrap();
        } else {
            contents.push_str(filename);
        }
        let response = format!("HTTP/1.1 {} OK\r\n\r\n{}", status, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        //wait to flush until ensuring the operation finished

        //println!("Request: {}", String::from_utf8_lossy(&buffer));
        //lossy define the behavior when see an invalid sequense - replace it with �, U+FFFD REPLACEMENT CHARACTER
    }
}

pub fn run_single_thread() {
    let listner = TcpListener::bind(URI).unwrap();
    //connect TCP connection on 127.0.0.1:7878
    println!("Server Successfully started. see http://localhost:7878");

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
        handle_connection(stream);
    }

    //TcpStream trace the data internally, so stream should be mutable even if read only
    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024]; //its enough in most cases
        let mut contents = String::new();

        stream.read(&mut buffer).unwrap(); //read from some stream

        let root = b"GET / HTTP/1.1\r\n";
        let js = b"GET /index.js HTTP/1.1\r\n";
        let css = b"GET /index.css HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

        let (status, filename) = if buffer.starts_with(root) {
            (200, "html/index.html")
        } else if buffer.starts_with(js) {
            (200, "html/index.js")
        } else if buffer.starts_with(css) {
            (200, "html/index.css")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_millis(5000));
            (200, "Im sleeping now")
        } else {
            (404, "Invalid")
        };

        if filename.contains(".") {
            File::open(filename)
                .unwrap()
                .read_to_string(&mut contents)
                .unwrap();
        } else {
            contents.push_str(filename);
        }
        let response = format!("HTTP/1.1 {} OK\r\n\r\n{}", status, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        //wait to flush until ensuring the operation finished

        println!("Request: {}", String::from_utf8_lossy(&buffer));
        //lossy define the behavior when see an invalid sequense - replace it with �, U+FFFD REPLACEMENT CHARACTER
    }
}
