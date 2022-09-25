use std::{fs};
use tera::{Tera};
use serde::{Serialize};
use std::fs::File;
use std::io::{Write};
use pulldown_cmark::{Parser, Options, html};
use yaml_rust::{Yaml, YamlLoader};
use crate::file_handler::{FileHandler};

extern crate tera;

const POSTS_FOLDER: &str = "posts";
pub const BUILD_FOLDER: &str = "build";
pub const TEMPLATE_FOLDER: &str = "templates";
const POST_PER_PAGE: usize = 2;

#[derive(Serialize, Clone, Debug)]
pub struct Post{
    title: String,
    date: String,
    summary: String,
    content: String,
    author: String,
    tags: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct IndexPost{
    title: String,
    title_slug: String,
    summary: String,
    date: String,
    tags: Vec<String>,
}


impl Post{
    pub fn new(post_name: &str) -> Post{
        let post_path = format!("{}/{}/{}.md", POSTS_FOLDER, post_name, post_name);
        let example = fs::read_to_string(post_path)
            .expect("Couldn't read the file");
        let (yaml_data, post_content) = Generator::parse_frontmatter(example.as_str());
        let docs: Vec<Yaml> = YamlLoader::load_from_str(&yaml_data).unwrap();
        let tags_return = docs[0]["tags"].clone();
        let mut tags: Vec<String> = Vec::new();
        for tag in tags_return{
            tags.push(tag.as_str().unwrap().to_string());
        }
        return Post{
            title: docs[0]["title"].as_str().unwrap().to_string(),
            date: docs[0]["date"].as_str().unwrap().to_string(),
            summary: docs[0]["summary"].as_str().unwrap().to_string(),
            content: Generator::md_to_html(post_content),
            author: docs[0]["author"].as_str().unwrap().to_string(),
            tags: tags,
        };
    }
}

pub struct Generator{
    tera: tera::Tera,
}

impl Generator{
    /// Returns a Tera instance
    pub fn new() -> Generator{
        let mut tera = match Tera::new("templates/*") {
            Ok(file) => file,
            Err(error) => panic!("Problem with glob: {:?}", error),
        };
        tera.autoescape_on(vec![]);
        Generator{tera}
    }

    /// Generates post to the $BUILD folder
    pub fn generate(self, call_from_livereload_: bool){
        let posts: Vec<String> = FileHandler::get_posts();
        let mut post_indexes: Vec<IndexPost> = Vec::new();
        FileHandler::create_folder(&format!("{}/{}", BUILD_FOLDER, POSTS_FOLDER));
        for post in &posts{
            FileHandler::create_folder(&format!("{}/{}/{}/", BUILD_FOLDER, POSTS_FOLDER, post));
            let output_path = format!("{}/{}/{}/{}.html", BUILD_FOLDER, POSTS_FOLDER, post, post);
            FileHandler::move_content(format!("{}/{}", POSTS_FOLDER, post),
                                      format!("{}/{}/{}", BUILD_FOLDER, POSTS_FOLDER, post),
                                      "md",
            );
            let new_post = Post::new(post.as_str());
            let mut context = tera::Context::new();
            context.insert("post", &new_post);
            let output = self.tera.render("post.html", &context).expect("Couldn't render context to template");
            Generator::write_to_a_file(&output_path, output);
            post_indexes.push(IndexPost{title: new_post.title, title_slug: post.to_string(), summary: new_post.summary, date: new_post.date, tags: new_post.tags});
        }
        if call_from_livereload_ == false{
            println!("successfully generated {} post(s)", posts.len());
        }
        self.get_index(&mut post_indexes);
    }

    fn write_to_a_file(path: &str, output: String){
        // moves css files in templates to $BUILD folder
        FileHandler::move_content(TEMPLATE_FOLDER.to_string(), BUILD_FOLDER.to_string(), "html");
        let mut file = File::create(path).expect("Couldn't create the output fille");
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
    /// Returns table of content made up from [h2..h6] of given string containing HTML
    fn get_table_of_content(html: String) -> String{
        return String::from("");
    
}
    // gets the post names and generates main page for them
    fn get_index(self, posts: &mut Vec<IndexPost>){
        posts.sort_by_key(|d| d.date.clone());
        let mut page_counter = 1;
        let max_page_counter = (posts.len() as f32/ POST_PER_PAGE as f32).ceil() as i32;
        loop{
            if posts.len() >= POST_PER_PAGE{
                let mut page_posts: Vec<IndexPost> = Vec::new();
                page_posts.push(posts.pop().expect("error"));
                page_posts.push(posts.pop().expect("error"));
                let mut context = tera::Context::new();
                context.insert("posts", &page_posts);
                context.insert("page_counter", &(page_counter));
                context.insert("max_page_counter", &max_page_counter);
                let output = self.tera.render("index.html", &context).expect("Couldn't render context to template");
                let output_path = format!("{}/{}.html", BUILD_FOLDER, get_index_name(page_counter)); 
                Generator::write_to_a_file(&output_path, output);
            }
            else{
                if posts.len() > 0{
                    let post = posts.clone();
                    let mut context = tera::Context::new();
                    context.insert("posts", &post);
                    context.insert("page_counter", &(page_counter));
                    context.insert("max_page_counter", &max_page_counter);
                    let output = self.tera.render("index.html", &context).expect("Couldn't render context to template");
                    let output_path = format!("{}/{}.html", BUILD_FOLDER, get_index_name(page_counter)); 
                    Generator::write_to_a_file(&output_path, output);
                }
                break;
            }
            page_counter += 1;

        }

    }
}


// improve it later
fn get_index_name(num: i32) -> String{
    return format!("index{}", num);
}







