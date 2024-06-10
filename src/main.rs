#[allow(unused_imports)]
use std::io::{self, Write};
#[allow(unused_imports)]
use std::{
    alloc::System,
    env,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn main() {
    let built_in_commands: [&str; 4] = ["echo", "exit", "type", "none"];
    // let mut system_paths = Vec::new();

    let key = "PATH";
    // let paths = env::var_os(key);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if !input.is_empty() {
            let mut system_paths: Vec<PathBuf> = Vec::new();

            match env::var_os(key) {
                Some(paths) => {
                    for path in env::split_paths(&paths) {
                        // println!("'{}'", path.display());
                        system_paths.push(path);
                    }
                }
                None => println!("Path not found"),
            }
            system_paths.push(env::current_dir().unwrap());

            if input == "exit 0" {
                break;
            }

            let parameters: Vec<&str> = input.split(" ").collect();

            if parameters[0] == "echo" {
                println!("{}", &parameters[1..].join(" "));
            } else if parameters[0] == "pwd" {
                println!("{}", system_paths.last().unwrap().to_str().unwrap());
            } else if parameters[0] == "cd" {
                let mut new_dir = PathBuf::from(system_paths.last().unwrap());
                new_dir.push(parameters[1]);
            } else if parameters[0] == "type" {
                let mut found = false;
                for command in built_in_commands {
                    if command == parameters[1] {
                        println!("{} is a shell builtin", command);
                        found = true;
                        break;
                    }
                }

                if !found {
                    for mut path in system_paths {
                        // println!("'{}'", path.display());
                        path.push(parameters[1]);
                        if path.exists() {
                            println!("{} is {}", parameters[1], path.to_str().unwrap());
                            found = true;
                            break;
                        }
                    }
                }

                if !found {
                    println!("{} not found", &parameters[1]);
                }
            } else {
                let mut found = false;
                for mut path in system_paths {
                    // println!("'{}'", path.display());
                    path.push(parameters[0]);
                    if path.exists() {
                        let mut cmd = Command::new(path);
                        cmd.args(&parameters[1..]);

                        match cmd.output() {
                            Ok(o) => unsafe {
                                print!("{}", String::from_utf8_unchecked(o.stdout));
                            },
                            Err(e) => {
                                println!("Error: {}", e)
                            }
                        }
                        found = true;
                        break;
                    }
                }

                if !found {
                    print!("{}: command not found\n", parameters[0]);
                }
            }
        }
    }
}
