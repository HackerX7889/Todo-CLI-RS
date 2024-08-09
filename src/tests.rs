// src/tests/integration_test.rs
#![cfg(test)]
use crate::Todo;

#[test]
fn test_todo_functionality() {
    let mut todo = Todo::new();

    // Test adding tasks
    todo.add_task("Task 1".to_string());
    todo.add_task("Task 2".to_string());
    assert_eq!(todo.tasks.len(), 2);
    assert_eq!(todo.tasks[0], "Task 1");
    assert_eq!(todo.tasks[1], "Task 2");

    // Test viewing tasks
    let tasks_before_removal = todo.tasks.clone();
    assert_eq!(tasks_before_removal, vec!["Task 1", "Task 2"]);

    // Test removing a task
    todo.remove_task(0);
    assert_eq!(todo.tasks.len(), 1);
    assert_eq!(todo.tasks[0], "Task 2");

    // Test removing a task with an invalid index
    todo.remove_task(10); // Should not panic
    assert_eq!(todo.tasks.len(), 1); // Still should be 1

    // Test loading from file
    todo.save_to_file().expect("Failed to save tasks");
    let mut todo_loaded = Todo::new();
    todo_loaded.load_from_file().expect("Failed to load tasks");
    assert_eq!(todo_loaded.tasks, vec!["Task 2"]);
}

#[test]
fn test_empty_todo() {
    let todo = Todo::new();
    assert_eq!(todo.tasks.len(), 0);
}

#[test]
fn test_remove_task_invalid_index() {
    let mut todo = Todo::new();
    todo.add_task("Task 1".to_string());
    todo.remove_task(1); // Invalid index
    assert_eq!(todo.tasks.len(), 1); // Should still be 1
}

#[test]
fn test_view_tasks_empty() {
    let todo = Todo::new();
    let output = todo.view_tasks(); // Should indicate no tasks available
    assert_eq!(output, ());
}
