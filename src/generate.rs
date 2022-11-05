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


extern crate tera;

#[derive(Serialize, Clone, Debug)]
pub struct Post{
    title_slug: String,
    content: String,
    date: String,
    info: HashMap<std::string::String, Vec<String>>,

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
        for hash_key in all_data.as_hash().unwrap().keys(){
            let key = hash_key.clone().into_string().unwrap();
            let value = &docs[0][key.as_str()].clone();
            if value.is_array(){
                let mut inner_values = Vec::new();
                for inner_value in docs[0]["tags"].clone(){
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
        //println!("{:?}", info);
        Ok(Post{title_slug, content, date, info})
    }
}

pub struct Generator{
    tera: tera::Tera,
    config: InkerConfig,
}

impl Generator{
    pub fn new() -> Generator{
        let src = InkerConfig::template_folder() + "/*";
        let mut tera = match Tera::new(src.as_str()) {
            Ok(file) => file,
            Err(error) => panic!("Problem with glob: {:?}", error),
        };
        tera.autoescape_on(vec![]);
        let config = InkerConfig::new();
        Generator{tera, config}
    }

    /// Generates post to the $BUILD folder
    pub fn generate(&mut self, call_from_livereload_: bool){
        FileHandler::create_folder(&format!("{}/{}", InkerConfig::build_folder(), "static"));
        FileHandler::move_content(format!("content/static"), format!("build/static"),"md");
        let posts_names: Vec<String> = FileHandler::get_posts();
        let mut posts: Vec<Post> = Vec::new();
        FileHandler::create_folder(&format!("{}/{}", InkerConfig::build_folder(), InkerConfig::posts_folder()));
        let mut generated_posts: i32 = 0;
        for post in &posts_names{
            FileHandler::create_folder(&format!("{}/{}/{}/", InkerConfig::build_folder(), InkerConfig::posts_folder(), post));
            let output_path = format!("{}/{}/{}/index.html", InkerConfig::build_folder(), InkerConfig::posts_folder(), post);
            let image_path = format!("{}/{}/{}/", InkerConfig::build_folder(), InkerConfig::posts_folder(), post);
            FileHandler::move_content(format!("{}/{}", InkerConfig::posts_folder(), post), image_path,"md");
            let new_post = Post::new(post.as_str()).unwrap_or_else(|err| {
                println!("inker failed: {err}");
                process::exit(1);
            });
            if new_post.info["draft"][0] == "false".to_string(){
                continue;
            }
            generated_posts += 1;
            let mut context = tera::Context::new();
            context.insert("post", &new_post);
            context.insert("icon_path", &self.config.icon_path);
            let output = self.tera.render("post.html", &context).expect("Couldn't render context to template");
            Generator::write_to_a_file(&output_path, output);
            posts.push(new_post);
        }
        if call_from_livereload_ == false{
            println!("successfully generated {} post(s)", generated_posts);
        }
        self.generate_extra();
        self.get_index(&mut posts);
    }

    pub fn generate_extra(&mut self){
        for content_info in &self.config.extra_contents{
            let src_path = format!("{}/{}", InkerConfig::content_folder(), content_info.content_src);
            let md_content = fs::read_to_string(src_path).expect("Couldn't read the file");
            let content = Generator::md_to_html(md_content);
            let mut context = tera::Context::new();
            context.insert("content", &content);
            context.insert("title", &content_info.title);
            context.insert("icon_path", &self.config.icon_path);
            let output = self.tera.render(content_info.template_src.as_str(), &context).expect("Couldn't render context to template");
            let output_path = format!("{}/{}/index.html", InkerConfig::build_folder(), content_info.title);
            FileHandler::create_folder(format!("build/{}", content_info.title).as_str());
            Generator::write_to_a_file(&output_path, output);
        }
    }

    fn write_to_a_file(path: &str, output: String){
        // moves css files in templates to $BUILD folder
        FileHandler::move_content(InkerConfig::template_folder().to_string(), InkerConfig::build_folder().to_string() + "/static", "html");
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
    

    // gets the post names and generates main page for them
    fn get_index(&mut self, posts: &mut Vec<Post>){
        posts.sort_by_key(|d| d.date.clone());
        if self.config.pagination == false{
            let mut context = tera::Context::new();
            context.insert("pagination_enabled", &false);
            context.insert("website_name", &self.config.website_name);
            context.insert("posts", &posts);
            context.insert("icon_path", &self.config.icon_path);
            context.insert("contents", &self.config.extra_contents);
            let output = self.tera.render("index.html", &context).expect("Couldn't render context to template");
            let output_path = format!("{}/index.html", InkerConfig::build_folder()); 
            Generator::write_to_a_file(&output_path, output);
        }
        else{
            let mut page_counter = 1;
            let max_page_counter = (posts.len() as f32/ self.config.posts_per_page as f32).ceil() as i32;
            let mut posts_left = true;
            FileHandler::create_folder(format!("{}/page", InkerConfig::build_folder()).as_str());
            while posts_left{
                let mut context = tera::Context::new();
                context.insert("pagination_enabled", &true);
                context.insert("website_name", &self.config.website_name);
                context.insert("page_counter", &(page_counter));
                context.insert("max_page_counter", &max_page_counter);
                context.insert("icon_path", &self.config.icon_path);
                context.insert("contents", &self.config.extra_contents);
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
                    let output_path = format!("{}/index.html", InkerConfig::build_folder());
                    Generator::write_to_a_file(&output_path, output.clone());
                }
                FileHandler::create_folder(format!("{}/page/{}", InkerConfig::build_folder(), page_counter).as_str());
                let output_path = format!("{}/page/{}/index.html", InkerConfig::build_folder(), page_counter);
                Generator::write_to_a_file(&output_path, output);
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



