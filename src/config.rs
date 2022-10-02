use std::time::SystemTime;
use chrono::{DateTime, Utc};
use std::{fs};
use yaml_rust::{Yaml, YamlLoader};
use serde::{Serialize, Deserialize};

pub struct InkerConfig{
    pub website_name: String,
    pub posts_per_page: i32,
    pub pagination: bool,
    pub icon_path: String,
    pub extra_contents: Vec<ContentInfo>,
}

const DEFAULT_CONFIG: &str = r#"website-name: "inker website"
posts-per-page: "4"
pagination: "false"
icon-path: "none"
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
            println!("config.yaml doesn't exist, getting the default content");
            config_content = DEFAULT_CONFIG.to_string();
        }
        let configs: Vec<Yaml> = YamlLoader::load_from_str(&config_content).unwrap();
        let config = &configs[0];
        let website_name = config["website-name"].as_str().unwrap_or("inker website").to_string();
        let posts_per_page: i32 = config["posts-per-page"].as_str().unwrap_or("4").to_string().parse().unwrap();
        let pagination: bool = config["pagination"].as_str().unwrap_or("false").to_string().parse().unwrap();
        let icon_path: String = config["icon-path"].as_str().unwrap_or("none").to_string().parse().unwrap();
        let extra =  &config["extra"];
        let mut extra_contents: Vec<ContentInfo> = Vec::new();
        for content in extra.as_vec().unwrap(){
            let src = content.as_hash().unwrap().front().unwrap().0.as_str().unwrap();
            let template = content.as_hash().unwrap().front().unwrap().1.as_str().unwrap();
            let visible_name = content.as_hash().unwrap().back().unwrap().1.as_str().unwrap();
            extra_contents.push(ContentInfo::new(src.to_string(), template.to_string(), visible_name.to_string()));
        }
        InkerConfig{website_name, posts_per_page, pagination, icon_path, extra_contents}
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