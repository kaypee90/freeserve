// Ref: https://doc.rust-lang.org/book/ch20-02-multithreaded.html

use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

mod lib;

use crate::lib::threadpool;

use log::{ info, trace, warn };
use simple_logger;

static IPADDRESS: &str = "0.0.0.0";
static PORT: &str = "7878";

fn main() { 
    simple_logger::init().unwrap();

    let socket = format!("{}:{}", IPADDRESS, PORT);

    let listener = TcpListener::bind(&socket).unwrap();
    let pool = match threadpool::ThreadPool::new(4) {
        Ok(pool) => pool,
        Err(_) => {
            warn!("Creating threadpool failed this might cause server to stop serving");
            panic!("Error while trying to create threadpool");
        }   
    };

    info!("Server running on {}!", socket);

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