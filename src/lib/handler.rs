use crate::lib::httpcode;

pub struct Handler {

} 

impl Handler{
    pub fn execute(identifier: &str) ->(&str, &str){
        let s: String = identifier.to_string().into();
    
        match &s[..] {
            "hello" => get_hello(),
            "about" => get_about(),
            _ => ("404.html", httpcode::HttpCode::status_404_not_found()),
        }
    }
}

fn get_hello() -> (&'static str, &'static str, ) {
    return ("hello.html", httpcode::HttpCode::status_200_ok());
}


fn get_about() -> (&'static str, &'static str, ){
    return ("about.html", httpcode::HttpCode::status_200_ok());
}
