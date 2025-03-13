use std::io::{self, Write};
use std::process::{Command, Child};
use std::env;
use std::collections::VecDeque;

const MAX_HISTORY: usize = 10;

struct Shell {
    history: VecDeque<String>,
}

impl Shell {
    fn new() -> Self {
        Shell {
            history: VecDeque::with_capacity(MAX_HISTORY),
        }
    }

    fn add_to_history(&mut self, command: &str) {
        if self.history.len() == MAX_HISTORY {
            self.history.pop_front();
        }
        self.history.push_back(command.to_string());
    }

    fn print_history(&self) {
        for (i, cmd) in self.history.iter().enumerate() {
            println!("[{}] {}", i + 1, cmd);
        }
    }

    fn handle_cd(&self, args: &[&str]) {
        if args.len() < 2 {
            eprintln!("cd: expected argument");
        } else if let Err(err) = env::set_current_dir(args[1]) {
            eprintln!("cd failed: {}", err);
        }
    }

    fn built_in_programs(&self, args: &[&str]) {
        match args[0] {
            "hello" => println!("Hello, welcome to my shell!"),
            "add" => {
                if args.len() > 2 {
                    if let (Ok(a), Ok(b)) = (args[1].parse::<i32>(), args[2].parse::<i32>()) {
                        println!("Sum: {}", a + b);
                    } else {
                        println!("Invalid numbers");
                    }
                } else {
                    println!("Usage: add <num1> <num2>");
                }
            }
            "subtract" => {
                if args.len() > 2 {
                    if let (Ok(a), Ok(b)) = (args[1].parse::<i32>(), args[2].parse::<i32>()) {
                        println!("Difference: {}", a - b);
                    } else {
                        println!("Invalid numbers");
                    }
                } else {
                    println!("Usage: subtract <num1> <num2>");
                }
            }
            _ => println!("Unknown command: {}", args[0]),
        }
    }

    fn execute_external(&self, args: &[&str], is_background: bool) {
        let mut command = Command::new(args[0]);
        command.args(&args[1..]);
        
        match command.spawn() {
            Ok(mut child) => {
                if !is_background {
                    let _ = child.wait();
                } else {
                    println!("Process running in background with PID: {:?}", child.id());
                }
            }
            Err(e) => eprintln!("Execution failed: {}", e),
        }
    }

    fn execute_command(&mut self, input: &str) {
        let mut args: Vec<&str> = input.trim().split_whitespace().collect();
        if args.is_empty() {
            return;
        }

        let is_background = if args.last() == Some(&"&") {
            args.pop();
            true
        } else {
            false
        };

        self.add_to_history(input);

        match args[0] {
            "cd" => self.handle_cd(&args),
            "history" => self.print_history(),
            "hello" | "add" | "subtract" => self.built_in_programs(&args),
            "quit" => {
                println!("Thank you for using the shell...\nBye 😉");
                std::process::exit(0);
            }
            _ => self.execute_external(&args, is_background),
        }
    }
}

fn main() {
    let mut shell = Shell::new();
    println!("Welcome to ⚡Blitz ⚡!! \nThis is a custom shell implemented in Rust!\nSay hello and wait for magic to happen 😄");

    loop {
        print!("my_shell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        
        if let Some('\n') = input.chars().last() {
            input.pop();
        }
        
        shell.execute_command(&input);
    }
}