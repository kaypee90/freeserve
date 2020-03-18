use log::{ info };
use simple_logger;

pub struct Router {
    
}

impl Router {
    pub fn get_route(buffer: &mut [u8]) -> (&str, &str) {

        // **** Add your routes here ****

        let routes = [
            vec!["GET", "/", "hello.html"],
            vec!["GET", "/about", "about.html"],
        ];

        // *** ----------------------- ***

        const HTTP_VERSION: &str = "HTTP/1.1\r\n";
        static _STATUS_200_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
        static _STATUS_404_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    
        let (mut status_line, mut body) = (_STATUS_404_NOT_FOUND, "404.html");
    
        for route in &routes {
            let http_method = route[0];
            let http_uri = route[1];
            let http_resource = route[2];

            let get = format!("{} {} {}", http_method, http_uri, HTTP_VERSION);

            if buffer.starts_with(get.as_bytes()) {
                status_line = _STATUS_200_OK;
                body = http_resource;
                info!("{} {} {}.", http_method, http_uri, status_line);
                break;
            }
        }
    
        return (status_line, body);
    }
    
}