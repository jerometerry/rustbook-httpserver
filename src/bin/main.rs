extern crate httpserver;

use httpserver::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {
    let addr = "127.0.0.1:8080";
    let max_workers = 4;

    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(max_workers);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| { handle_connection(stream); });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let data = read(&mut stream);
    let (status, filename) = get_response(data);
    let html = get_file_contents(filename);
    write_response(stream, status, &html);
}

fn get_response(buffer: [u8; 512]) -> (&'static str, &'static str) {
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        ("200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "hello.html")
    } else {
        ("404 NOT FOUND", "404.html")
    }
}

fn read(mut stream: &TcpStream) -> [u8; 512] {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    buffer
}

fn write_response(mut stream: TcpStream, status: &str, contents: &str) {
    let response = format_response(status, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn format_response(status: &str, contents: &str) -> String {
    let http_version = "HTTP/1.1";
    format!("{} {}\r\n\r\n{}", http_version, status, contents)
}

fn get_file_contents(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}