use std::{env};
use std::fs;



fn main() {
    let args: Vec<String> = env::args().collect();

    let git_command: &String = &args[1];
    let git_args: Vec<String> = args[2..].to_vec();

    println!("Running git {} {:?}", git_command, git_args);
}
