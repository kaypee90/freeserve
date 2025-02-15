use log::{ info };

use crate::lib::handler::Handler;
use crate::lib::httpcode::HttpCode;
use crate::lib::config::Response;
use crate::lib::config::ContentType;

pub struct Router {
    
}

impl Router {
    pub fn get_route(buffer: &mut [u8]) -> Vec<u8> {

        // **** Add your routes here ****

        let routes = [
            vec!["POST", "/", "hello"], // Format: [HTTP_METHOD, PATH, UNIQUE_NAME]
            vec!["GET", "/about", "about"],
        ];

        // *** ----------------------- ***

        let http_version: &str = HttpCode::http_version();
        let status_400_not_found: &str = HttpCode::status_404_not_found();
        let (mut body, mut status_line) = (String::from("404.html"), status_400_not_found);
        let mut int_status_code: u16 = 404;

        let request_body = String::from_utf8_lossy(&buffer[..]);
        let iter_request = request_body.split("\n");

        // **********************************************************
        // THIS CODE IS FOR RETRIEVING POST BODY FROM THE TCP STREAM

        let vec: Vec<&str> = iter_request.collect();
        let number_of_params = vec.len();

        let mut body = String::new(); // post body
        let mut params = String::new(); // query parameters

        let mut http_method = String::new();
        let mut http_uri = String::new();

        let mut counter = 0;
        for item in &vec {
            if !item.contains(":") && counter > 1{
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
            let uri = vec_http_info[1].to_string();
            let query_parameters = uri.split("?");
            let query_string: Vec<&str> = query_parameters.collect();
            
            if query_string.len() > 1 // If request has valid request query strings
            {
                http_uri = query_string[0].to_string();
                params = query_string[1].to_string();
            }
            else{
                http_uri = vec_http_info[1].to_string();
            }
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
            if route[0].to_string() == http_method && route[1].to_string() == http_uri {
                let (http_resource, status_code) = Handler::execute(body, params, route_identifier);
                status_line = status_code;
                body = http_resource;
                int_status_code = 200;
                info!("{} {} {}.", http_method, http_uri, status_line);
                break;
            } else {
                info!("No matching route found!");
                body = String::new();
            }
        }
    
        return build_response(int_status_code, http_method, body);
    }
}

fn build_response(int_status_code: u16, http_method: String, response_body: String) -> Vec<u8>{
    let mut result = format!(
        "HTTP/1.0 {} \n",
        int_status_code.to_string(),
    );

    result = format!("{}Allow: {}\n", result, http_method);

    let mut response = Response::new();
    response.status = int_status_code;
    response.headers.content_type = Some(ContentType::from_app_config());

    match response.headers.content_type {
        Some(content_type) => {
            result = format!(
                "{}Content-type: {}\n", result, content_type.value());
        },
        _ => (),
    }

    let mut bytes = result.as_bytes().to_vec();

    match response_body {
        body => {
            bytes.append(&mut "\n".as_bytes().to_vec());
            bytes.append(&mut body.as_bytes().to_vec());
        },
        _ => (),
    }

    bytes
}
