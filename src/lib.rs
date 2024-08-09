//! # To-Do List CLI
//!
//! A simple command-line to-do list application written in Rust.
//! This crate allows users to manage tasks efficiently through a terminal interface.
//!
//! ## Features
//! - Add tasks to the list
//! - Remove tasks by index
//! - View all tasks
//! - Persistent storage of tasks using a text file
//!
//! ## Usage
//!
//! To use this crate, you can create a new instance of the `Todo` struct and call its methods:
//!
//! ```
//! use todo_cli_rs::Todo;
//!
//! let mut todo = Todo::new();
//! todo.add_task("Learn Rust".to_string());
//! todo.view_tasks();
//! todo.remove_task(0);
//! ```
//!
//! ## Running the Application
//!
//! To run the command-line application, use the following command:
//! ```bash
//! cargo run
//! ```
//!
//! After starting the application, you can interact with it through the terminal to manage your tasks.

pub mod todo;
mod tests;

pub use todo::Todo;
