use std::fs;
use log::{ info };
use crate::lib::config;

const BASE_PATH: &str = "/usr/src/project/";
const DEFAULT_TEMPLATE_PATH: &str = "src/templates/";

pub struct ActionResult {

}

impl ActionResult{

    pub fn view<'a> (resource: String, status_code: &'a str) -> (String, &'a str){
        let mut content = resource.to_string();
        let app_config = config::Config::new();
        let fullpath = format!("{}{}{}", BASE_PATH, DEFAULT_TEMPLATE_PATH, resource); 

        content = match app_config.use_api_view {
            false => fs::read_to_string(fullpath).unwrap(),
            _ => content
        };

        return (content, status_code);
    }
}