use log::{ info };

use crate::lib::handler;
use crate::lib::httpcode;

pub struct Router {
    
}

impl Router {
    pub fn get_route(buffer: &mut [u8]) -> (String, &str) {

        // **** Add your routes here ****

        let routes = [
            vec!["GET", "/", "hello"], // Format: [HTTP_METHOD, PATH, UNIQUE_NAME]
            vec!["GET", "/about", "about"],
        ];

        // *** ----------------------- ***

        let http_version: &str = httpcode::HttpCode::http_version();
        let status_400_not_found: &str = httpcode::HttpCode::status_404_not_found();
        let (mut body, mut status_line) = (String::from("404.html"), status_400_not_found);
    
        for route in &routes {
            let http_method = route[0];
            let http_uri = route[1];
            let route_identifier = route[2];
           
            let get = format!("{} {} {}", http_method, http_uri, http_version);

            if buffer.starts_with(get.as_bytes()) {
                let (http_resource, status_code) = handler::Handler::execute(route_identifier);
                status_line = status_code;
                body = http_resource;
                info!("{} {} {}.", http_method, http_uri, status_line);
                break;
            }
        }
    
        return (body, status_line);
    }
    
}