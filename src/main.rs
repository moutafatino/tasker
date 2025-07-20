use std::io::{self, Write};

use task::{Task, TaskManager};

mod task;
fn main() {
    let mut app = TaskManager::init();

    let mut input = String::new();

    println!("Welcome to Task Manager!");
    println!("Type 'help' for available commands or 'quit' to exit.");

    loop {
        print!("tm> ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input!");

        if input.trim() == "quit" {
            return;
        } else if input.trim() == "help" {
            app.cmd_help();
            println!("Good bye!");
        }

        input.clear()
    }
}
