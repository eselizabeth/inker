use crate::generate::{Generator, BUILD_FOLDER, TEMPLATE_FOLDER};
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
        else if command == "deleteall".to_string()
        || command == "build".to_string()
        || command == "clean".to_string() {
            argument = "".to_string();
        }
        else{
            return Err("you entered an unknown command");
        }
        let all_args = args.clone();
        Ok(Cli{command, argument})
    }
    pub fn handle_input(&self){
        FileHandler::initalize();
        if self.command == "build"{
            FileHandler::remove_folder_content(BUILD_FOLDER.to_string());
            Generator::generate();
        }
        else if self.command == "clean"{
            FileHandler::remove_folder_content(BUILD_FOLDER.to_string());
        }
        else if self.command == "new"{
            FileHandler::create_post(self.argument.clone());
        }
        else if self.command == "delete"{
            FileHandler::delete_post(self.argument.clone());
        }
        else if self.command == "deleteall"{
            FileHandler::remove_folder_content(BUILD_FOLDER.to_string());
            FileHandler::remove_folder_content("posts".to_string());
            println!("sucessfully deleted all content");
        }
        else{
            println!("command not found: {}", self.command);
        }
    }
}
