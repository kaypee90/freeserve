pub struct HttpCode {

}

impl HttpCode {
    pub fn http_version() -> &'static str{
        return "HTTP/1.1\r\n";
    }
    
    pub fn status_200_ok() -> &'static str{
        return "HTTP/1.1 200 OK\r\n\r\n";
    }

    pub fn status_404_not_found() -> &'static str{
        return "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    }
}

