pub struct Config {
    use_api_view: bool,
}

impl Config {
    pub fn new() -> Config {
        let app_config = Config {
            use_api_view : false,
        };

        return app_config;
    }

    pub fn is_api_view(&self) -> bool {
        return self.use_api_view;
    }
}