use std::io::ErrorKind;
use crate::generate::{Generator};
use crate::file_handler::{FileHandler};
use crate::config::{InkerConfig};

use crate::webserver::{run_server};


const CURRENT_COMMANDS: [&'static str; 6] = ["build", "clean", "new", "delete", "deleteall", "livereload"];


pub struct Cli<'a>{
    command: String,
    all_args: &'a [String],
}

impl Cli<'_>{
    pub fn new(args: &[String]) -> Result<Cli, &'static str>{
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let command = args[1].clone();
        if command == "new".to_string()
        || command == "delete".to_string()
        {
            if args.len() < 3{
                return Err("not enough arguments, please provide the post name for deletion process");
            }
        }
        if !CURRENT_COMMANDS.contains(&command.as_str()){
            return Err("you have entered an unknown command. current commands are \nbuild\nclean\nnew\ndelete\ndeleteall\nlivereload");
        }
        let all_args = args;
        Ok(Cli{command, all_args})
    }
    pub fn handle_input(&self){
        FileHandler::initalize();
        if self.command == "build"{
            FileHandler::remove_folder_content(InkerConfig::build_folder().to_string());
            let mut generator = Generator::new();
            generator.generate();
            run_server().expect("couldn't start the server");
        }
        else if self.command == "clean"{
            FileHandler::remove_folder_content(InkerConfig::build_folder().to_string());
        }
        else if self.command == "new"{
            let full_name = &self.all_args[2..self.all_args.len()].join(" ");
            FileHandler::create_post(full_name.to_string());
        }
        else if self.command == "delete"{
            let full_name = &self.all_args[2..self.all_args.len()].join(" ");
            FileHandler::delete_post(full_name.to_string());
        }
        else if self.command == "deleteall"{
            let mut user_input = String::new();
            println!("are you sure you want to delete all the posts? [y/n]");
            std::io::stdin().read_line(&mut user_input).unwrap();
            user_input.pop().unwrap(); // to remove the \n
            if user_input == "y" || user_input == "yes"{
                FileHandler::remove_folder_content(InkerConfig::build_folder().to_string());
                FileHandler::remove_folder_content("posts".to_string());
                println!("all content has been deleted");
            }
            else if user_input == "n" || user_input == "no"{
                println!("deletion aborted");
            }
            else{
                println!("please enter yes or no");
            }
        }
        else if self.command == "livereload"{
            FileHandler::remove_folder_content(InkerConfig::build_folder().to_string());
            let mut generator = Generator::new();
            generator.generate();
            println!("watching for changes..");
            match run_server(){
                Ok(()) => (),
                Err(error) => match error.kind() {
                    ErrorKind::AddrInUse => println!("this port is already in use: {}", InkerConfig::new().port),
                    _ => println!("{}", error)
                }            }
        }
    }
}
