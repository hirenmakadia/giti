use std::{env};
use std::collections::HashMap;
use iterator::enumerate;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config: GitConfig = GitConfig::new( &args)
    );

    println!("Running git {} {:?}", git_command, git_args);
}


struct GitConfig {
    command: String,
    args: Option<Vec<String>>,
    kwargs: Option<HashMap<String, String>>,
}

impl GitConfig {
    fn new(args: &[String]) -> Result<GitConfig, &'static str>{
        if args.len() == 0 {
            return Err("Not enough arguments");
        }

        let command = args[1].clone();
        let mut kwargs = HashMap::new();
        let mut args = Vec::new();

        let is_kwarg: bool = false;

        for (ind, arg) in enumerate(args.iter()) {
            if (arg.starts_with("--") or arg.starts_with("-")) or is_kwarg {
                if is_kwarg {
                    kwargs.insert(key, arg);
                    is_kwarg = false;
                }
                else {
                    let key = arg.clone();
                    is_kwarg = true;
                }
            }
            else {
                args.push(arg);
            }
        }

        if is_kwarg() {
            return Err("Invalid arguments for flag {}", key);
        }

        GitConfig {
            command,
            args,
            flag_arg,
        }
    }
}