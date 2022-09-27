use std::time::SystemTime;
use chrono::{DateTime, Utc};
use std::{fs};
use yaml_rust::{Yaml, YamlLoader};

pub struct InkerConfig{
    pub website_name: String,
    pub posts_per_page: i32,
}

const DEFAULT_CONFIG: &str = r#"website-name: "inker website"
posts-per-page: "4""#;

impl InkerConfig{
    pub fn new() -> InkerConfig{
        let config_file = fs::read_to_string("config.yaml");
        let config_content: String;
        if config_file.is_ok(){
            config_content = config_file.unwrap();
        }
        else{
            println!("config.yaml doesn't exist, getting the default content");
            config_content = DEFAULT_CONFIG.to_string();
        }
        let config: Vec<Yaml> = YamlLoader::load_from_str(&config_content).unwrap();
        let website_name = config[0]["website-name"].as_str().unwrap_or("inker website").to_string();
        let posts_per_page: i32 = config[0]["posts-per-page"].as_str().unwrap_or("4").to_string().parse().unwrap();
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
        /// returns the default post template
    pub fn post_template() -> String{
            let template = format!(r#"---
title: "title"
date: "{}"
summary: "summary"
author: "author"
tags: [tag1, tag2]
---
Enter your content here"#, InkerConfig::current_time());
            return template;
        }
            /// returns the current time in ISO 8601
    fn current_time() -> String{
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now_iso = now.to_rfc3339();
        return now_iso;
    }
}