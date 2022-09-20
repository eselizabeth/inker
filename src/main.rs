pub mod generate;

use crate::generate::{generate};

use std::{env};

fn main() {
    // Currently this program only takes a file name and creates of html for with according to the given template
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    generate(file.clone());
}
