use std::{fs};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use slugify::slugify;
use crate::config::InkerConfig;


pub struct FileHandler{
}

impl FileHandler{
    /// Creates the following folders if they don't exist in the current directory
    pub fn initalize(){
        let _ = FileHandler::create_folder(InkerConfig::posts_folder());
        let _ = FileHandler::create_folder(InkerConfig::publish_folder());
        let _ = FileHandler::create_folder(InkerConfig::build_folder());
        let _ = FileHandler::create_folder(InkerConfig::content_folder());
        let _ = FileHandler::create_folder(&(InkerConfig::content_folder().to_owned() + &"/static".to_string()));
    }

    /// For /posts folder creates a folder and .md file inside it with the slug version of given name
    pub fn create_post(post_name: String){
        let post_slug = slugify!(&post_name.clone());
        let folder_name = format!("{}/{}", InkerConfig::posts_folder(), post_slug);
        let file_name = format!("{}/{}", folder_name, (post_slug.clone() + ".md"));
        let result: bool = FileHandler::create_folder(&folder_name);
        if result == false{
            println!("Please try an another name, the post with given name already exists: {}", post_name.clone());
        }
        else{
            let mut file = File::create(file_name).expect("Couldn't create the output file");
            let post_template = InkerConfig::post_template();
            write!(file, "{post_template}").expect("Couldn't write to the output file");
            println!("The post has been successfully created: {}", post_name);
        }

    }

    /// For /posts folder deletes folder with the given name
    pub fn delete_post(post_name: String){
        let post_slug = slugify!(&post_name.clone());
        let folder_name = format!("{}/{}", InkerConfig::posts_folder(), post_slug);
        let result: bool = FileHandler::delete_folder(&folder_name);
        if result == true{
            println!("The post has been successfully deleted");
        }
        else if result == false{
            println!("The post with given name doesn't exist: {}", post_name);
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
        for folder in fs::read_dir(InkerConfig::posts_folder()).unwrap() {
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
        if Self::folder_existence(folder_name){
            fs::remove_dir_all(folder_name).expect("Couldn't delete the file");
            return true;
        }
        else{
            return false;
        }
    }
    // Returns true if the folder with given path exists, otherwise will return false
    pub fn folder_existence(folder_name: &str) -> bool{
        return Path::new(folder_name).is_dir();
    }

    /// Iterates over the from_folder folder ands moves each file that doesn't fit to the filter to to_folder folder
    pub fn move_content(from_folder: String, to_folder: String, filter: &str){
        let mut other_files: Vec<String> = Vec::new();
        let files = fs::read_dir(from_folder.clone()).unwrap();
        files
            .filter_map(Result::ok)
            .filter(|d| if let Some(e) = d.path().extension() { e != filter } else {false})
            .for_each(|f| other_files.push(f.file_name().into_string().expect("Error on moving file")));
        for file in other_files{
            let from = format!("{}/{}", from_folder, file);
            let to = format!("{}/{}", to_folder, file);
            fs::copy(from, to).expect("Couldn't move the content file");
        }
    }

}


