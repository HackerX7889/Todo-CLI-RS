use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead};
use std::path::Path;

use colored::Colorize;

/// A struct representing a to-do list.
#[derive(Debug)]
pub struct Todo {
    pub tasks: Vec<String>,
}

impl Todo {
    /// Creates a new empty `Todo` list.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_cli_rs::Todo;
    /// let todo = Todo::new();
    /// assert_eq!(todo.tasks.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Adds a task to the to-do list.
    /// 
    /// # Arguments
    /// 
    /// * `task` - A string representing the task to be added.
    ///
    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
        self.save_to_file().expect("Failed to save tasks");
    }

    /// Removes a task from the to-do list by index.
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the task to be removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_cli_rs::Todo;
    /// let mut todo = Todo::new();
    /// todo.add_task("Task 1".to_string());
    /// todo.remove_task(0);
    /// assert_eq!(todo.tasks.len(), 0);
    /// ```
    pub fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            self.save_to_file().expect("Failed to save tasks");
        } else {
            println!("Invalid task index.");
        }
    }

    /// Displays all tasks in the to-do list.
    pub fn view_tasks(&self) {
        if self.tasks.is_empty() {
            println!("{}", "No tasks available.".yellow());
        } else {
            println!("{:<5} {:<20}", "Index", "Task");
            println!("{}", "=".repeat(30));
            for (i, task) in self.tasks.iter().enumerate() {
                println!("{:<5} {:<20}", i, task);
            }
        }
    }

    /// Saves the current tasks to a file.
    pub fn save_to_file(&self) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).create(true).open("tasks.txt")?;
        file.set_len(0)?; // Clear the file
        for task in &self.tasks {
            writeln!(file, "{}", task)?;
        }
        Ok(())
    }

    /// Loads tasks from a file into the to-do list.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_cli_rs::Todo;
    /// let mut todo = Todo::new();
    /// todo.load_from_file().expect("Failed to load tasks");
    /// ```
    pub fn load_from_file(&mut self) -> io::Result<()> {
        if Path::new("tasks.txt").exists() {
            let file = fs::File::open("tasks.txt")?;
            let reader = io::BufReader::new(file);
            self.tasks.clear();
            for line in reader.lines() {
                self.tasks.push(line?);
            }
        }
        Ok(())
    }
}