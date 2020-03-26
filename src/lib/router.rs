use log::{ info };

use crate::lib::handler;
use crate::lib::httpcode;

pub struct Router {
    
}

impl Router {
    pub fn get_route(buffer: &mut [u8]) -> (String, &str) {

        // **** Add your routes here ****

        let routes = [
            vec!["POST", "/", "hello"], // Format: [HTTP_METHOD, PATH, UNIQUE_NAME]
            vec!["GET", "/about", "about"],
        ];

        // *** ----------------------- ***

        let http_version: &str = httpcode::HttpCode::http_version();
        let status_400_not_found: &str = httpcode::HttpCode::status_404_not_found();
        let (mut body, mut status_line) = (String::from("404.html"), status_400_not_found);

        let request_body = String::from_utf8_lossy(&buffer[..]);
        let iter_request = request_body.split("\n");

        // **********************************************************
        // THIS CODE IS FOR RETRIEVING POST BODY FROM THE TCP STREAM

        let vec: Vec<&str> = iter_request.collect();
        let number_of_params = vec.len();
        println!("Number of Splits: {}", number_of_params);

        let mut body = String::new(); // post body
        let mut params = String::new(); // query parameters

        let mut http_method = String::new();
        let mut http_uri = String::new();

        let mut counter = 0;
        for item in &vec {
            if !item.contains(":") && counter > 1{
                println!("{}", item);
                break;
            }
            counter = counter + 1;
        }

        let last_header = vec[counter-1]; // post body comes after last header
        let http_info = vec[0];
        let iter_http_info = request_body.split(" ");
        let vec_http_info: Vec<&str> = iter_http_info.collect();
        
        http_method = vec_http_info[0].to_string();
        if vec_http_info.len() > 1{
            http_uri = vec_http_info[1].to_string();
        }
        

        if http_info.contains("POST"){
            let mut post_request = request_body.split(last_header); //split request based on last header to get post body
            let post_vec: Vec<&str> = post_request.collect();
            
            if post_vec.len() > 1{
                body = post_vec[1].trim().to_string();
            }
        } 

        // **********************************************************
    
        for route in &routes {
            let route_identifier = route[2];
           
            //let get = format!("{} {} {}", http_method, http_uri, http_version);

            if route[0].to_string() == http_method && route[1].to_string() == http_uri {
                let (http_resource, status_code) = handler::Handler::execute(body, params, route_identifier);
                status_line = status_code;
                body = http_resource;
                info!("{} {} {}.", http_method, http_uri, status_line);
                break;
            } else {
                body = String::new();
            }
        }
    
        return (body, status_line);
    }
    
}