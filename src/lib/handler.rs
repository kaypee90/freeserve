use crate::lib::httpcode;
use crate::lib::actionresult;


pub struct Handler {

} 

impl Handler{
    pub fn execute(body: String, params: String, identifier: &str) ->(String, &str){
        let s: String = identifier.to_string().into();
    
        match &s[..] {
            "hello" => get_hello(body, params),
            "about" => get_about(body, params),
            _ => actionresult::ActionResult::view("404.html", httpcode::HttpCode::status_404_not_found()),
        }
    }
}

fn get_hello(body: String, params: String) -> (String, &'static str, ) {
    println!("Print ActionContent Body:: {}", body);
    return actionresult::ActionResult::view("hello.html", httpcode::HttpCode::status_200_ok());
}


fn get_about(body: String, params: String) -> (String, &'static str, ){
    return actionresult::ActionResult::view("about.html", httpcode::HttpCode::status_200_ok());
}
