use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

mod lib;

use crate::lib::threadpool;

fn main() { 
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = threadpool::ThreadPool::new(4);

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });   
        
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let fullpath = format!("/usr/src/project/{}", filename); 
    let contents = fs::read_to_string(fullpath).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}