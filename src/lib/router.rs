use log::{ info };
use simple_logger;

pub struct Router {
    
}

impl Router {
    pub fn get_route(buffer: &mut [u8]) -> (&str, &str) {
        const HTTP_VERSION: &str = "HTTP/1.1\r\n";

        // **** Add your routes here ****

        let routes = [
            vec!["GET", "/", "hello.html"],
            vec!["GET", "/about", "about.html"],
        ];

        // *** ----------------------- ***

        let _status_200_ok = "HTTP/1.1 200 OK\r\n\r\n";
        let _status_404_not_found = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    
        let (mut status_line, mut body) = (_status_404_not_found, "404.html");
    
        for route in &routes {
            let http_method = route[0];
            let http_uri = route[1];
            let http_resource = route[2];

            let get = format!("{} {} {}", http_method, http_uri, HTTP_VERSION);
            if buffer.starts_with(get.as_bytes()) {
                status_line = _status_200_ok;
                body = http_resource;
                info!("{} {} {}.", http_method, http_uri, status_line);
                break;
            }
        }
    
        return (status_line, body);
    }
    
}