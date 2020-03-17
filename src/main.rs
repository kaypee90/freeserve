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
const HTTP_VERSION: &str = "HTTP/1.1\r\n";

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

    let  (mut status_line, mut filename) = get_route(&mut buffer);

    let fullpath = format!("/usr/src/project/{}", filename); 
    let contents = fs::read_to_string(fullpath).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_route(buffer: &mut [u8]) -> (&str, &str) {
    let status_200_ok = "HTTP/1.1 200 OK\r\n\r\n";
    let status_404_not_found = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

    let routes = [
        vec!["GET", "/", "hello.html"],
        vec!["GET", "/about", "about.html"],
    ];

    let (mut status_line, mut filename) = (status_404_not_found, "404.html");

    for route in &routes {
        let get = format!("{} {} {}", route[0], route[1], HTTP_VERSION);
        if buffer.starts_with(get.as_bytes()) {
            status_line = "HTTP/1.1 200 OK\r\n\r\n";
            filename = route[2];
            break;
        }
    }

    return (status_line, filename);
}
