#[allow(unused_imports)]
use std::io::{self, Write};
#[allow(unused_imports)]
use std::{alloc::System, env, path::Path};

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

        if input == "exit 0" {
            break;
        }

        let parameters: Vec<&str> = input.split(" ").collect();

        if parameters[0] == "echo" {
            println!("{}", &parameters[1..].join(" "));
        } else if parameters[0] == "type" {
            for command in built_in_commands {
                if command == parameters[1] {
                    println!("{} is a shell builtin", command);
                    break;
                }

                match env::var_os(key) {
                    Some(paths) => {
                        for mut path in env::split_paths(&paths) {
                            // println!("'{}'", path.display());
                            path.push(parameters[1]);
                            if path.exists() {
                                println!("{} is {}", parameters[1], path.to_str().unwrap());
                                break;
                            }
                        }
                        println!("{} not found", &parameters[1]);
                        break;
                    }
                    None => println!("{} not found", &parameters[1]),
                }
            }
        } else {
            print!("{}: command not found\n", parameters[0]);
        }
    }
}
