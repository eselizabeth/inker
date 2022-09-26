use std::{fs};
use yaml_rust::{Yaml, YamlLoader};

pub struct InkerConfig{
    pub website_name: String,
    pub posts_per_page: i32,
}

impl InkerConfig{
    pub fn new() -> InkerConfig{
        let config_content = fs::read_to_string("config.yaml").expect("Couldn't read the file");
        let config: Vec<Yaml> = YamlLoader::load_from_str(&config_content).unwrap();
        let website_name = config[0]["website-name"].as_str().unwrap().to_string();
        let posts_per_page: i32 = config[0]["posts-per-page"].as_str().unwrap().to_string().parse().unwrap();
        InkerConfig{website_name, posts_per_page}
    }
    pub fn build_folder() -> &'static str{
        return "build";
    }
    pub fn posts_folder() -> &'static str{
        return "posts";
    }
    pub fn template_folder() -> &'static str{
        return "templates";
    }
}