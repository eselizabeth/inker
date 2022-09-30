use crate::generate::{Generator};
use crate::file_handler::{FileHandler};
use crate::config::{InkerConfig};

extern crate notify;

use notify::{Watcher, RecursiveMode, Config, RecommendedWatcher, EventKind};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;
use crate::webserver::{run_server};


const CURRENT_COMMANDS: [&'static str; 6] = ["build", "clean", "new", "delete", "deleteall", "livereload"];


pub struct Cli<'a>{
    command: String,
    all_args: &'a [String],
}

impl Cli<'_>{
    pub fn new(args: &[String]) -> Result<Cli, &'static str>{
        if args.len() < 2 {
            return Err("not enough arg2s");
        }
        let command = args[1].clone();
        if command == "new".to_string()
        || command == "delete".to_string()
        {
            if args.len() < 3{
                return Err("not enough arguments, please provide the post name");
            }
        }
        if !CURRENT_COMMANDS.contains(&command.as_str()){
            return Err("you entered an unknown command. current commands are \nbuild\nclean\nnew\ndelete\ndeleteall");
        }
        let all_args = args.clone();
        Ok(Cli{command, all_args})
    }
    pub fn handle_input(&self){
        FileHandler::initalize();
        if self.command == "build"{
            FileHandler::remove_folder_content(InkerConfig::build_folder().to_string());
            let generator = Generator::new();
            generator.generate(false);
            run_server();
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
                println!("sucessfully deleted all content");
            }
            else if user_input == "n" || user_input == "no"{
                println!("deletion aborted");
            }
            else{
                println!("please enter yes or no");
            }
        }
        else if self.command == "livereload"{
            println!("watching for changes..");
            let (sender, receiver) = channel();
            let mut watcher: RecommendedWatcher = Watcher::new(sender, Config::default()
            .with_poll_interval(Duration::from_secs(5))
            .with_compare_contents(true)).unwrap();
            watcher.watch(Path::new(InkerConfig::template_folder()), RecursiveMode::Recursive).unwrap();
            watcher.watch(Path::new(InkerConfig::posts_folder()), RecursiveMode::Recursive).unwrap();
            watcher.watch(Path::new("config.yaml"), RecursiveMode::Recursive).unwrap();

            loop {
                match receiver.recv() {
                Ok(event) => match event.unwrap().kind {
                    EventKind::Access(_) => {
                        println!("changes found, reloading");
                        let generator = Generator::new();
                        generator.generate(true);
                    }
                    _ => (),
                },
                Err(e) => println!("watch error {:?}", e),
                }
            }
        }
    }
}
