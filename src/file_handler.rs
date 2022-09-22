use std::{fs};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use slugify::slugify;

pub const BUILD_FOLDER: &str = "build";
const POSTS_FOLDER: &str = "posts";
const TEMPLATE_FOLDER: &str = "templates";


const POST_TEMPLATE: &str = r#"---
title: "title"
date: 1 Jan 1970
author: "author"
tags: [tag1, tag2]
---
Enter your content here"#;



pub struct FileHandler{
}

impl FileHandler{
    /// Creates posts and templates folder if they don't exist
    pub fn initalize(){
        let _ = FileHandler::create_folder(POSTS_FOLDER);
        let _ = FileHandler::create_folder(TEMPLATE_FOLDER);
    }

    /// For /posts folder creates a folder and .md file inside it with the slug version of given name
    pub fn create_post(post_name: String){
        let post_slug = slugify!(&post_name.clone());
        let folder_name = format!("{}/{}", POSTS_FOLDER, post_slug);
        let file_name = format!("{}/{}", folder_name, (post_slug.clone() + ".md"));
        let result: bool = FileHandler::create_folder(&folder_name);
        if result == false{
            println!("this post already exists: {}", post_name.clone());
        }
        else{
            let mut file = File::create(file_name).expect("Couldn't create the output file");
            write!(file, "{POST_TEMPLATE}").expect("Couldn't write to the output file");
            println!("successfully created post: {}", post_name);
        }

    }

    /// For /posts folder deletes folder with the given name
    pub fn delete_post(post_name: String){
        let post_slug = slugify!(&post_name.clone());
        let folder_name = format!("{}/{}", POSTS_FOLDER, post_slug);
        let result: bool = FileHandler::delete_folder(&folder_name);
        if result == true{
            println!("successfully deleted post: {}", post_name);
        }
        else if result == false{
            println!("this post doesn't exist: {}", post_name);
        }
    }

    /// Iterates over the folder and deletes the content
    pub fn remove_folder_content(path: String){
        for entry in fs::read_dir(path).unwrap() {
            //println!("{:?}", entry.unwrap().path());
            if entry.as_ref().unwrap().file_type().unwrap().is_dir()  {
                fs::remove_dir_all(entry.as_ref().unwrap().path()).expect("Couldn't delete the folder");
            }
            else if entry.as_ref().unwrap().file_type().unwrap().is_file()  {
                fs::remove_file(entry.as_ref().unwrap().path()).expect("Couldn't remove file");
        }
        }
    }

    /// Iterates over the folder and deletes the content
    pub fn get_posts() -> Vec<String>{
        let mut posts: Vec<String> = Vec::new();
        for folder in fs::read_dir(POSTS_FOLDER).unwrap() {
            if folder.as_ref().unwrap().file_type().unwrap().is_dir()  {
                    let post_name = folder.as_ref().unwrap().file_name().into_string().unwrap();
                    posts.push(post_name);
                    // folder.as_ref().unwrap().path().display().to_string()
                }
            }
        return posts;
    }
    

    /// Creates a folder: returns false if the folder already exists
    pub fn create_folder(folder_name: &str) -> bool{
        let folder_exists: bool = Path::new(folder_name).is_dir();
        if !folder_exists{
            fs::create_dir(folder_name).expect("Couldn't create the folder");
            return true;
        }
        else{
            return false;
        }
    }

    /// Creates a folder: returns false if the folder doesn't exists
    pub fn delete_folder(folder_name: &str) -> bool{
        let folder_exists: bool = Path::new(folder_name).is_dir();
    if folder_exists{
            fs::remove_dir_all(folder_name).expect("Couldn't delete the file");
            return true;
        }
        else{
            return false;
        }
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
}
