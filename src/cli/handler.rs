use std::io;

use crate::models::todo::ToDoItem;

pub fn print_todos(todos: &mut Vec<ToDoItem>) {
    for todo in todos.iter() {
        println!("{}", todo.string_repr());
        println!("{}", checkbox(todo.is_complete()));
    }

    if let Some(todo) = todos.get_mut(0) {
        todo.complete();
    }

    for todo in todos.iter() {
        println!("{}", todo.string_repr());
        println!("{}", checkbox(todo.is_complete()));
    }
}

pub fn checkbox(condition: bool) -> &'static str {
    if condition {
        "☑"
    } else {
        "☐"
    }
}

pub fn show_options() {
    println!("----- WELCOME -----");
    println!("Select an option: ");
    println!("1. View todo items");
    println!("2. Add a todo item");
    println!("3. Check off a todo item");
    println!("Quit");
}

pub fn get_option() -> i32 {
    let valid_options = vec![1, 2, 3];

    loop {
        let mut selected_option = String::new();
        io::stdin().read_line(&mut selected_option).expect("Line could not be read");
        let selected_option: i32 = selected_option.trim().parse().unwrap_or(-1);

        if selected_option == -1 {
            continue
        }

        if valid_options.contains(&selected_option) {
            return selected_option
        }
    }
}

pub fn main_loop() {
    show_options();

    loop {
        let mut selected_option = String::new();
        io::stdin().read_line(&mut selected_option).expect("Line could not be read");
        let selected_option = selected_option.trim().to_string();

        let selected_option: i32 = selected_option.parse().unwrap();

        let mut valid_options = vec![1, 2, 3];

        if valid_options.contains(&selected_option) {

        }
        
    }
}