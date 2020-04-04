// Ref: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

mod lib;

use crate::lib::threadpool::ThreadPool;
use crate::lib::router::Router;

use log::{ info, trace };
use simple_logger;

static IPADDRESS: &str = "0.0.0.0";
static PORT: &str = "7878";

fn main() { 
    simple_logger::init().unwrap();

    let socket = format!("{}:{}", IPADDRESS, PORT);

    let listener = TcpListener::bind(&socket).unwrap();
    let pool = match ThreadPool::new(4) {
        Ok(pool) => pool,
        Err(_) => {
            trace!("Creating threadpool failed this might cause server to stop serving");
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

    let (contents, status_line) = Router::get_route(&mut buffer);
    let response = format!("{}{}", status_line, &contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

