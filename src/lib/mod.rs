pub mod threadpool;
pub mod router;
pub mod handler;
pub mod httpcode;
pub mod config;
pub mod actionresult;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_thread_pull_with_valid_size() {
        let is_valid = match threadpool::ThreadPool::new(4) {
            Ok(_pool) => true,
            Err(_) => false,    
        };

        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_config_is_api_view() {
        let mut app_config = config::Config::new();
        app_config.use_api_view = true;

        assert_eq!(app_config.use_api_view, true);
    }

    #[test]
    fn test_httpcode_http_version(){
        let http_version = httpcode::HttpCode::http_version();

        assert_eq!(http_version, "HTTP/1.1\r\n");
    }

    #[test]
    fn test_httpcode_status_200_ok(){
        let status_200_ok = httpcode::HttpCode::status_200_ok();

        assert_eq!(status_200_ok, "HTTP/1.1 200 OK\r\n\r\n");
    }

    #[test]
    fn test_httpcode_status_404_not_found(){
        let status_404_not_found = httpcode::HttpCode::status_404_not_found();

        assert_eq!(status_404_not_found, "HTTP/1.1 404 NOT FOUND\r\n\r\n");
    }

    #[test]
    fn test_get_hello_handler(){
        let expected_response = actionresult::ActionResult::view("hello.html", httpcode::HttpCode::status_200_ok());
        let actual_response = handler::Handler::execute("hello");
        assert_eq!(expected_response, actual_response);
    }

    #[test]
    fn test_get_about_handler(){
        let expected_response = actionresult::ActionResult::view("about.html", httpcode::HttpCode::status_200_ok());
        let actual_response = handler::Handler::execute("about");
        assert_eq!(expected_response, actual_response);
    }

    #[test]
    fn test_invalid_handler(){
        let expected_response = actionresult::ActionResult::view("404.html", httpcode::HttpCode::status_404_not_found());
        let actual_response = handler::Handler::execute("invalid");
        assert_eq!(expected_response, actual_response);
    }

}
