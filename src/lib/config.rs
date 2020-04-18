pub struct Config {
   pub use_api_view: bool,
}

impl Config {
    pub fn new() -> Config {
        let app_config = Config {
            use_api_view : false,
        };

        return app_config;
    }
}

pub enum ContentType {
    JSON,
    HTML,
}

impl ContentType {
    pub fn from_app_config () -> ContentType {
        let app_config = Config::new();
        match app_config.use_api_view {
            true => ContentType::JSON,
            false => ContentType::HTML,
        }
    }

    pub fn value(&self) -> &str {
        match *self {
            ContentType::HTML => "text/html",
            ContentType::JSON => "application/json",
        }
    }
}

pub struct ResponseHeaders {
    pub content_type: Option<ContentType>,
}

impl ResponseHeaders {
    pub fn new() -> ResponseHeaders {
        ResponseHeaders {
            content_type: None,
        }
    }
}

pub struct Response {
    pub body: Option<Vec<u8>>,
    pub headers: ResponseHeaders,
    pub status: u16,
}

impl Response {
    pub fn new() -> Response {
        Response {
            body: None,
            headers: ResponseHeaders::new(),
            status: 200,
        }
    }
}
