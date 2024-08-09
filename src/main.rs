// src/main.rs
mod todo;
mod tests;

use std::io;
use std::io::Write;
pub use todo::Todo;
use colored::Colorize;

fn main() {
    let mut todo = Todo::new();
    todo.load_from_file().expect("Failed to load tasks");

    loop {
        // Clear the console for better readability (optional)
        print!("\x1B[2J\x1B[1;1H"); // ANSI escape codes to clear the screen
        println!(
            "{}",
            "==============================".cyan()
        );
        println!(
            "{}",
            "          To-Do List         ".cyan()
        );
        println!(
            "{}",
            "==============================".cyan()
        );

        println!(
            "{:<5} {:<20}",
            "1.", "Add Task".green()
        );
        println!(
            "{:<5} {:<20}",
            "2.", "Remove Task".green()
        );
        println!(
            "{:<5} {:<20}",
            "3.", "View Tasks".green()
        );
        println!(
            "{:<5} {:<20}",
            "4.", "Exit".green()
        );

        print!("\n{} ", "Enter your choice:".yellow());
        io::stdout().flush().expect("Failed to flush stdout");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim() {
            "1" => {
                print!("{} ", "Enter task:".yellow());
                io::stdout().flush().expect("Failed to flush stdout");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed to read line");
                todo.add_task(task.trim().to_string());
                println!("{}", "Task added successfully!".green());
            }
            "2" => {
                println!("{}", "Your tasks:".yellow());
                todo.view_tasks();
                print!("{} ", "Enter task index to remove:".yellow());
                io::stdout().flush().expect("Failed to flush stdout");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read line");
                if let Ok(idx) = index.trim().parse::<usize>() {
                    todo.remove_task(idx);
                    println!("{}", "Task removed successfully!".green());
                }
            }
            "3" => {
                println!("{}", "Your tasks:".yellow());
                todo.view_tasks();
                print!("Press Enter to continue...");
                let _ = io::stdin().read_line(&mut String::new());
            }
            "4" => break,
            _ => println!("{}", "Invalid choice, please try again.".red()),
        }
    }
}