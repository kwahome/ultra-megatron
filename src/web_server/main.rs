extern crate server_threadpool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;
use server_threadpool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    let mut counter = 0;

    for stream in listener.incoming() {
        if counter == 10 {
            println!("Shutting down.");
            break;
        }

        counter += 1;

        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}\n", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "res/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(20));
        ("HTTP/1.1 200 OK\r\n\r\n", "res/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "res/404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new(); // use of String rather than literals
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);
    println!("Response: {}\n", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
