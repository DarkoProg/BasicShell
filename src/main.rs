#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let built_in_commands: [&str; 4] = ["echo", "exit", "type", "none"];
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
            println!("{}", &parameters[1]);
        } else if parameters[0] == "type" {
            for command in built_in_commands {
                if command == parameters[1] {
                    println!("{} is a shell builtin", command);
                    break;
                }
                if command == "none" {
                    println!("{} not found", &parameters[1]);
                }
            }
        } else {
            print!("{}: command not found\n", parameters[0]);
        }
    }
}
