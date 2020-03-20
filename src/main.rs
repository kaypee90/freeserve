// Ref: https://doc.rust-lang.org/book/ch20-02-multithreaded.html

use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

mod lib;

use crate::lib::threadpool;
use crate::lib::router;

use log::{ info, trace };
use simple_logger;

static IPADDRESS: &str = "0.0.0.0";
static PORT: &str = "7878";
const BASE_PATH: &str = "/usr/src/project/";
const DEFAULT_TEMPLATE_PATH: &str = "src/templates/";

fn main() { 
    simple_logger::init().unwrap();

    let socket = format!("{}:{}", IPADDRESS, PORT);

    let listener = TcpListener::bind(&socket).unwrap();
    let pool = match threadpool::ThreadPool::new(4) {
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

    let  (status_line, filename) = router::Router::get_route(&mut buffer);

    let fullpath = format!("{}{}{}", BASE_PATH, DEFAULT_TEMPLATE_PATH, filename); 
    info!("Fetching file from: {}", fullpath);

    let contents = fs::read_to_string(fullpath).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

