use std::{fs};
use tera::{Tera};
use serde::{Serialize};
use std::fs::File;
use std::io::{Write};
use pulldown_cmark::{Parser, Options, html};
use yaml_rust::{Yaml, YamlLoader};
use crate::file_handler::{FileHandler};
use crate::config::{InkerConfig};
use std::{process};
use std::collections::HashMap;
use std::cmp::Reverse;

extern crate tera;

#[derive(Serialize, Clone, Debug)]
pub struct Post{
    title_slug: String,
    content: String,
    date: String,
    order: String, // because f64 doesn't implement Ord
    info: HashMap<std::string::String, Vec<String>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct MainItem{
    title: String,
    order: String,
    items: Vec<SubItem>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SubItem{
    title: String,
    title_slug: String,
    order: String,
}


impl MainItem{
    pub fn new(title: String) ->  Self{
        let items: Vec<SubItem> = Vec::new();
        let order = title.split('.').collect::<Vec<&str>>()[0].to_string();
        return MainItem{title, order, items}
    }
}

impl SubItem{
    pub fn new(title: String, title_slug: String, order: String) -> Self{
        return SubItem{title, title_slug, order}
    }
}


impl Post{
    pub fn new(post_name: &str) -> Result<Post, String>{
        let title_slug = post_name.to_string();
        let post_path = format!("{}/{}/{}.md", InkerConfig::posts_folder(), post_name, post_name);
        let example = fs::read_to_string(post_path).expect("Couldn't read the file");
        let (yaml_data, post_content) = Generator::parse_frontmatter(example.as_str());
        let content = Generator::md_to_html(post_content);
        let docs: Vec<Yaml> = YamlLoader::load_from_str(&yaml_data).unwrap();
        let all_data = docs[0].clone();
        let mut info = HashMap::new();
        let order = docs[0]["order"].as_str().unwrap_or("9999.0").to_string();
        for hash_key in all_data.as_hash().unwrap().keys(){
            let key = hash_key.clone().into_string().unwrap();
            let value = &docs[0][key.as_str()].clone();
            if key == "order" || key == "date"{
                continue;
            }
            if value.is_array(){
                let mut inner_values = Vec::new();
                for inner_value in docs[0][key.as_str()].clone(){
                    inner_values.push(inner_value.into_string().unwrap());
                }
                info.insert(key, inner_values);
            }
            else{
                let mut inner_values = Vec::new();
                inner_values.push(value.clone().into_string().unwrap());
                info.insert(key, inner_values);
            }
        }
        let date = match docs[0]["date"].as_str().ok_or("couldn't convert option to result") {
            Ok(title) => title.to_string(),
            Err(_) => return Err(format!("couldn't find date in the post: {}", post_name)),
        };
        Ok(Post{title_slug, content, date, order, info})
    }
}

pub struct Generator{
    tera: tera::Tera,
    config: InkerConfig,
    for_publish: bool,
    output_folder: String
}

impl Generator{
    pub fn new(for_publish: bool) -> Generator{
        let src = InkerConfig::template_folder() + "/*";
        let mut tera = match Tera::new(src.as_str()) {
            Ok(file) => file,
            Err(error) => panic!("Problem with glob: {:?}", error),
        };
        tera.autoescape_on(vec![]);
        let mut config = InkerConfig::new().unwrap();
        let mut output_folder = InkerConfig::publish_folder().to_string();
        if !for_publish{
            let _ = &config.webserver_usage();
            output_folder = InkerConfig::build_folder().to_string();
        }

        Generator{tera, config, for_publish, output_folder}
    }

    /// Generates post to the $BUILD folder
    pub fn generate(&mut self){
        FileHandler::create_folder(&format!("{}/{}", self.output_folder, "static"));
        FileHandler::move_content(format!("content/static"), format!("{}/static", &self.output_folder),"md");
        let posts_names: Vec<String> = FileHandler::get_posts();
        let mut posts: Vec<Post> = Vec::new();
        FileHandler::create_folder(&format!("{}/{}", self.output_folder, InkerConfig::posts_folder()));
        let mut generated_posts: i32 = 0;
        let mut navigation: Vec<(String, String, String)> = Vec::new();
        for post in &posts_names{
            let new_post = Post::new(post.as_str()).unwrap_or_else(|err| {
                println!("inker failed: {err}");
                process::exit(1);
            });
            if new_post.info["draft"][0] == "true".to_string(){
                continue;
            }
            navigation.push((new_post.order.clone(), new_post.info["title"][0].clone(), new_post.title_slug.clone()));
            generated_posts += 1;
            FileHandler::create_folder(&format!("{}/{}/{}/", self.output_folder, InkerConfig::posts_folder(), post));
            let output_path = format!("{}/{}/{}/index.html", self.output_folder, InkerConfig::posts_folder(), post);
            let image_path = format!("{}/{}/{}/", self.output_folder, InkerConfig::posts_folder(), post);
            FileHandler::move_content(format!("{}/{}", InkerConfig::posts_folder(), post), image_path,"md");
            // tera inserts
            let mut context = tera::Context::new();
            context.insert("post", &new_post);
            context.insert("icon_path", &self.config.icon_path);
            context.insert("base_url", &self.config.base_url);
            let output = self.tera.render("post.html", &context).expect("Couldn't render context to template");
            self.write_to_a_file(&output_path, output);
            posts.push(new_post);
        }
        if generated_posts == 0{
            println!("there isn't any post to generate");
            process::exit(0);
        }
        self.generate_extra();
        self.get_index(&mut posts);
        if self.config.generate_nav {
            self.generate_navigation(&mut navigation);
        }
    }

    pub fn generate_extra(&mut self){
        for content_info in &self.config.extra_contents{
            let src_path = format!("{}/{}", InkerConfig::content_folder(), content_info.content_src);
            let md_content = fs::read_to_string(src_path).expect("Couldn't read the file");
            let content = Generator::md_to_html(md_content);
            // tera inserts
            let mut context = tera::Context::new();
            context.insert("content", &content);
            context.insert("title", &content_info.title);
            context.insert("icon_path", &self.config.icon_path);
            context.insert("base_url", &self.config.base_url);
            let output = self.tera.render(content_info.template_src.as_str(), &context).expect("Couldn't render context to template");
            let output_path = format!("{}/{}/index.html", &self.output_folder, content_info.title);
            FileHandler::create_folder(format!("{}/{}", &self.output_folder, content_info.title).as_str());
            self.write_to_a_file(&output_path.clone(), output.clone());
        }
    }

    fn write_to_a_file(&self, path: &str, output: String){
        // moves css files in templates to $BUILD folder
        FileHandler::move_content(InkerConfig::template_folder().to_string(), self.output_folder.to_string() + "/static", "html");
        let mut file = File::create(path).expect("Couldn't create the output file");
        write!(file, "{output}").expect("Couldn't write to the output file");
    
    }
    
    
    /// Takes a file to path and returns as YAML metadata section and the rest(post content)
    fn parse_frontmatter(input: &str) -> (String, String){
        let start_index = input.find("---").unwrap();
        let end_index = input[start_index + 3..].find("---").unwrap();
        return  (String::from(&input[start_index + 3..end_index + 2]),
                String::from(&input[end_index + 6..]));
    }
    
    /// Takes a string in MD form and returns it in HTML form
    fn md_to_html(contents: String) -> String{
        // Set up options and parser. Strikethroughs are not part of the CommonMark standard
        // and we therefore must enable it explicitly.
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&contents, options);
        // Write to String buffer.
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        return html_output;
    }
    

    fn generate_navigation(&mut self, posts: &mut Vec<(String, String, String)>){
        posts.sort_by_key(|p| p.0.clone());
        let mut navigator: Vec<MainItem> = Vec::new();
        let mut main_headers = self.config.headers.clone();
        let mut current_header = MainItem::new(main_headers.remove(0));
        for post in posts{
            if post.0.chars().nth(0) == current_header.order.chars().nth(0){
                // do nothing
            }
            else{
                navigator.push(current_header);
                if main_headers.len() > 0{
                    current_header = MainItem::new(main_headers.remove(0));
                }
                else{
                    current_header =  MainItem::new("none".to_string());
                }
                }
            let sub_item = SubItem::new(post.1.clone(), post.2.clone(), post.0.clone());
            current_header.items.push(sub_item.clone());
        }
        navigator.push(current_header);
        // tera inserts        
        let mut context = tera::Context::new();
        context.insert("navigator", &navigator.clone());
        context.insert("icon_path", &self.config.icon_path);
        context.insert("base_url", &self.config.base_url);
        let output = self.tera.render("nav_template.html", &context).expect("Couldn't render context to template");
        self.write_to_a_file(format!("{}/nav.html", &self.output_folder).as_str(), output);
        }
    

    // gets the post names and generates main page for them
    fn get_index(&mut self, posts: &mut Vec<Post>){
        posts.sort_by_key(|item| (item.order.clone(), Reverse(item.date.clone())));
        if self.config.pagination == false{
            // tera inserts
            let mut context = tera::Context::new();
            context.insert("pagination_enabled", &false);
            context.insert("website_name", &self.config.website_name);
            context.insert("posts", &posts);
            context.insert("icon_path", &self.config.icon_path);
            context.insert("contents", &self.config.extra_contents);
            context.insert("base_url", &self.config.base_url);
            let output = self.tera.render("index.html", &context).expect("Couldn't render context to template");
            let output_path = format!("{}/index.html", &self.output_folder); 
            self.write_to_a_file(&output_path, output);
        }
        else{
            let mut page_counter = 1;
            let max_page_counter = (posts.len() as f32/ self.config.posts_per_page as f32).ceil() as i32;
            let mut posts_left = true;
            FileHandler::create_folder(format!("{}/page", &self.output_folder).as_str());
            while posts_left{
                // tera inserts
                let mut context = tera::Context::new();
                context.insert("pagination_enabled", &true);
                context.insert("website_name", &self.config.website_name);
                context.insert("page_counter", &(page_counter));
                context.insert("max_page_counter", &max_page_counter);
                context.insert("icon_path", &self.config.icon_path);
                context.insert("contents", &self.config.extra_contents);
                context.insert("base_url", &self.config.base_url);
                if posts.len() >= self.config.posts_per_page as usize{
                    let page_posts: Vec<Post> = get_first_n_elements(posts, self.config.posts_per_page);
                    context.insert("posts", &page_posts);
                }
                else{
                    if posts.len() > 0{
                        let other_posts = posts.clone();
                        context.insert("posts", &other_posts);
                        posts_left = false;
                    }
                }
                let output = self.tera.render("index.html", &context).expect("Couldn't render context to template");
                if page_counter == 1{
                    let output_path = format!("{}/index.html", &self.output_folder);
                    self.write_to_a_file(&output_path, output.clone());
                }
                FileHandler::create_folder(format!("{}/page/{}", &self.output_folder, page_counter).as_str());
                let output_path = format!("{}/page/{}/index.html", &self.output_folder, page_counter);
                self.write_to_a_file(&output_path, output);
                page_counter += 1;
            }
    
        }
    }
}

/// gets the first n elements of vector containing Post
fn get_first_n_elements(vec: &mut Vec<Post>, n: i32) -> Vec<Post>{
    let mut n_vector: Vec<Post> = Vec::new();
    for _ in 0..n{
        n_vector.push(vec.pop().expect("error"));
    }
    return n_vector;
}



