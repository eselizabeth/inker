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

#[derive(Serialize)]
pub struct Post{
    title: String,
    date: String,
    content: String,
    author: String,
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
            content: Generator::md_to_html(post_content),
            author: docs[0]["author"].as_str().unwrap().to_string(),
            tags: tags,
        };
    }
}

pub struct Generator{

}

impl Generator{
    pub fn generate(){
        let mut tera = match Tera::new("templates/*") {
            Ok(file) => file,
            Err(error) => panic!("Problem with glob: {:?}", error),
        };
        tera.autoescape_on(vec![]);
        let posts: Vec<String> = FileHandler::get_posts();
        for post in &posts{
            let output_path = format!("{}/{}.html", BUILD_FOLDER, post); 
            let example_post = Post::new(post.as_str());
            let mut context = tera::Context::new();
            context.insert("post", &example_post);
            let output = tera.render("template.html", &context).expect("Couldn't render context to template");
            Generator::write_to_a_file(&output_path, output);
        }
        println!("successfully generated {} post(s)", posts.len());
    }

    pub fn write_to_a_file(path: &str, output: String){
        FileHandler::move_css();
        let mut file = File::create(path).expect("Couldn't create the output fille");
        write!(file, "{output}").expect("Couldn't write to the output file");
    
    }
    
    
    /// Takes a file to path and returns as YAML metadata section and the rest(post content)
    pub fn parse_frontmatter(input: &str) -> (String, String){
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
    pub fn get_table_of_content(html: String) -> String{
        return String::from("");
}
}









