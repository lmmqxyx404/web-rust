use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:65534").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("Hello, world!{:?}",stream);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 Not Found\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}Content-Length:{}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
