use std::time::SystemTime;
use chrono::{DateTime, Utc};
use std::{fs};
use yaml_rust::{Yaml, YamlLoader};
use serde::{Serialize, Deserialize};


pub struct InkerConfig{
    pub port: u16,
    pub website_name: String,
    pub template_name: String,
    pub posts_per_page: i32,
    pub generate_nav: bool,
    pub pagination: bool,
    pub icon_path: String,
    pub extra_contents: Vec<ContentInfo>,
    pub headers: Vec<String>,
}

const DEFAULT_CONFIG: &str = r#"port: 8080
website-name: "inker website"
posts-per-page: "4"
pagination: "false"
icon-path: "none"
generate_nav: "false"
"#;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentInfo{
    pub content_src: String,
    pub template_src: String,
    pub title: String,
}

impl ContentInfo{
    pub fn new(content_src: String, template_src: String, title: String,) -> ContentInfo{
        ContentInfo{content_src, template_src, title}
    }
}

impl InkerConfig{
    pub fn new() -> InkerConfig{
        let config_file = fs::read_to_string("config.yaml");
        let config_content: String;
        if config_file.is_ok(){
            config_content = config_file.unwrap();
        }
        else{
            config_content = DEFAULT_CONFIG.to_string();
            println!("config.yaml file doesn't exist, the default configuration will be used:\n{}", {config_content.clone()});
        }
        let configs: Vec<Yaml> = YamlLoader::load_from_str(&config_content).unwrap();
        let config = &configs[0];
        let port: u16 = config["port"].as_str().unwrap_or("8080").to_string().parse().unwrap();
        let website_name = config["website-name"].as_str().unwrap_or("inker website").to_string();
        let template_name: String = config["template-name"].as_str().unwrap_or("bs-darkly").to_string().parse().unwrap();
        let posts_per_page: i32 = config["posts-per-page"].as_str().unwrap_or("4").to_string().parse().unwrap();
        let pagination: bool = config["pagination"].as_str().unwrap_or("false").to_string().parse().unwrap();
        let generate_nav: bool = config["generate-nav"].as_str().unwrap_or("false").to_string().parse().unwrap();
        let icon_path: String = config["icon-path"].as_str().unwrap_or("none").to_string().parse().unwrap();
        let extra =  &config["extra"];
        let mut extra_contents: Vec<ContentInfo> = Vec::new();
        if !extra.is_badvalue(){
            for content in extra.as_vec().unwrap(){
                let src = content.as_hash().unwrap().front().unwrap().0.as_str().unwrap();
                let template = content.as_hash().unwrap().front().unwrap().1.as_str().unwrap();
                let visible_name = content.as_hash().unwrap().back().unwrap().1.as_str().unwrap();
                extra_contents.push(ContentInfo::new(src.to_string(), template.to_string(), visible_name.to_string()));
            }
        }
        let header_values =  &config["headers"];
        let mut headers: Vec<String> = Vec::new();
        if !header_values.is_badvalue(){
            for header in header_values.as_vec().unwrap(){
                headers.push(header.as_str().unwrap().to_string());
            }
        }
        InkerConfig{port, website_name, template_name, posts_per_page, generate_nav, pagination, icon_path, extra_contents, headers}
    }
    pub fn build_folder() -> &'static str{
        return "build";
    }
    pub fn content_folder() -> &'static str{
        return "content";
    }
    pub fn posts_folder() -> &'static str{
        return "posts";
    }
    pub fn template_folder() -> std::string::String{
        let template_name = InkerConfig::new().template_name;
        return "templates/".to_string() + &template_name.to_string().clone();
    }
    /// returns the default post template [from template/model.yaml]
    pub fn post_template() -> String{
        let mut model = fs::read_to_string("templates/".to_string() + &InkerConfig::new().template_name + "/model.yaml").expect("model.yaml is missing!");
        let rest = format!("\ndate: {} # this field is mandatory to have \n---\nenter your content here", InkerConfig::current_time());
        model = "---\n".to_string() + &model + &rest;
        return model;
        }
    /// returns the current time in ISO 8601 format
    pub fn current_time() -> String{
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now_iso = now.to_rfc3339();
        return now_iso;
    }
}