use std::{fs};
use std::path::Path;


pub struct FileHandler{
}

impl FileHandler{
    pub fn delete_folder(folder_name: &str){
        let build_exists: bool = Path::new(folder_name).is_dir();
        if build_exists{
            fs::remove_dir_all(folder_name).expect("Couldn't delete the file");
        }
    }
    /// Erases the content of /posts folder
    pub fn clean_posts(){
        println!("stub");
    }
    /// For /posts folder creates a folder  and .md file inside it with the given name
    pub fn create_post(post_name: String){
        println!("stub");
    }
    /// For /posts folder creates deletes folder with the given name
    pub fn delete_post(post_name: String){
        println!("stub");
    }
}