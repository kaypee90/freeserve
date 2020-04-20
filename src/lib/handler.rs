use crate::lib::httpcode::HttpCode;
use crate::lib::actionresult::ActionResult;


pub struct Handler {

} 

impl Handler{
    pub fn execute(body: String, params: String, identifier: &str) ->(String, &str){
        let s: String = identifier.to_string().into();
    
        match &s[..] {
            "hello" => post_hello(body, params),
            "about" => get_about(body, params),
            _ => ActionResult::view("404.html".to_string(), HttpCode::status_404_not_found()),
        }
    }
}

fn post_hello(body: String, params: String) -> (String, &'static str, ) {

    // return ActionResult::view("hello.html".to_string(), HttpCode::status_200_ok());
    return ActionResult::view("{\"message\":\"Hello World\"}".to_string(), HttpCode::status_200_ok());
}


fn get_about(body: String, params: String) -> (String, &'static str, ){

    println!("Query Parameters: {}", params);
    //return ActionResult::view("about.html".to_string(), HttpCode::status_200_ok());
    return ActionResult::view("{\"message\":\"About Free serve\"}".to_string(), HttpCode::status_200_ok());
}
