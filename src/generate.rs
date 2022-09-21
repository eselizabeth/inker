use std::{fs};
use tera::{Tera};
use serde::{Serialize};
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use pulldown_cmark::{Parser, Options, html};
use yaml_rust::{Yaml, YamlLoader};

extern crate tera;

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
    pub fn new(path: &str) -> Post{
        let example = fs::read_to_string(path)
        .expect("Couldn't read the file");
    let (yaml_data, post_content) = parse_frontmatter(example.as_str());
    let docs: Vec<Yaml> = YamlLoader::load_from_str(&yaml_data).unwrap();
    let tags_return = docs[0]["tags"].clone();
    let mut tags: Vec<String> = Vec::new();
    for tag in tags_return{
        tags.push(tag.as_str().unwrap().to_string());
    }
    return Post{
        title: docs[0]["title"].as_str().unwrap().to_string(),
        date: docs[0]["date"].as_str().unwrap().to_string(),
        content: md_to_html(post_content),
        author: docs[0]["author"].as_str().unwrap().to_string(),
        tags: tags,
    };
    }
}


pub fn generate(file_path: String){
    let example_post = Post::new(file_path.as_str());
    let mut tera = match Tera::new("templates/*") {
        Ok(file) => file,
        Err(error) => panic!("Problem with glob: {:?}", error),
    };
    tera.autoescape_on(vec![]);
    let mut context = tera::Context::new();
    context.insert("post", &example_post);
    let output = tera.render("template.html", &context).expect("Couldn't render context to template");
    write_to_a_file("output.html", output);

}


pub fn write_to_a_file(path: &str, output: String){
    // Remove the build folder and recreate it
    let build_exists: bool = Path::new(BUILD_FOLDER).is_dir();
    if build_exists{
        fs::remove_dir_all(BUILD_FOLDER).expect("Couldn't delete the file");
    }
    fs::create_dir(BUILD_FOLDER).expect("Couldn't create the folder");
    // **
    move_css();
    let output_file = format!("{}/{}", BUILD_FOLDER, path);
    let mut file = File::create(output_file).expect("Couldn't create the output fikle");
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


/// Iterates over the $TEMPLATES folder ands moves each .css file to $BUILD folder
pub fn move_css(){
    let mut css_files: Vec<String> = Vec::new();
    let files = fs::read_dir(TEMPLATE_FOLDER).unwrap();
    files
        .filter_map(Result::ok)
        .filter(|d| if let Some(e) = d.path().extension() { e == "css" } else {false})
        .for_each(|f| css_files.push(f.file_name().into_string().expect("Error on moving css")));
    for css_file in css_files{
        let from = format!("{}/{}", TEMPLATE_FOLDER, css_file);
        let to = format!("{}/{}", BUILD_FOLDER, css_file);
        fs::copy(from, to).expect("Couldn't move the css file");
    }
}

/// Returns table of content made up from [h2..h6] of given string containing HTML
pub fn get_table_of_content(html: String) -> String{
    return String::from("");
}