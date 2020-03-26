use crate::lib::httpcode;
use crate::lib::actionresult;


pub struct Handler {

} 

impl Handler{
    pub fn execute(body: String, params: String, identifier: &str) ->(String, &str){
        let s: String = identifier.to_string().into();
    
        match &s[..] {
            "hello" => post_hello(body, params),
            "about" => get_about(body, params),
            _ => actionresult::ActionResult::view("404.html".to_string(), httpcode::HttpCode::status_404_not_found()),
        }
    }
}

fn post_hello(body: String, params: String) -> (String, &'static str, ) {

    return actionresult::ActionResult::view("hello.html".to_string(), httpcode::HttpCode::status_200_ok());
}


fn get_about(body: String, params: String) -> (String, &'static str, ){
    return actionresult::ActionResult::view("about.html".to_string(), httpcode::HttpCode::status_200_ok());
}
