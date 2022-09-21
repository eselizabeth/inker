use crate::generate::{generate, BUILD_FOLDER, TEMPLATE_FOLDER};
use crate::file_handler::{FileHandler};

pub struct Cli{
    command: String,
    argument: String,
}

impl Cli{
    pub fn new(args: &[String]) -> Result<Cli, &'static str>{
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let command = args[1].clone();
        let argument: String; 
        if command == "new".to_string()
        || command == "delete".to_string()
        {
            if args.len() < 3{
                return Err("not enough arguments, please provide the post name");
            }
            else{
                argument = args[2].clone();
            }
        }
        else{
            argument = "".to_string();
        }
        Ok(Cli{command, argument})
    }
    pub fn handle_input(&self){
        if self.command == "build"{
            generate("output.html".to_string());
        }
        else if self.command == "clean"{
            FileHandler::delete_folder(BUILD_FOLDER);
        }
        else if self.command == "new"{
            FileHandler::create_post(self.argument.clone());
        }
        else if self.command == "delete"{
            FileHandler::delete_post(self.argument.clone());
        }
        else if self.command == "deletall"{
            FileHandler::delete_folder(BUILD_FOLDER);
            FileHandler::clean_posts();
        }
    }
}
