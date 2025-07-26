use std::{io, process};

use crate::models::todo::ToDoItem;

pub fn print_todos(todos: &Vec<ToDoItem>) {
    for todo in todos.iter() {
        println!("{}: {}", todo.string_repr(), checkbox(todo.is_complete()));
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
    println!("4. Quit");
}

pub fn get_option(valid_options: &[i32]) -> i32 
{
    loop {
        let mut selected_option = String::new();
        io::stdin().read_line(&mut selected_option).expect("Line could not be read");
        match selected_option.trim().parse::<i32>() {
            Ok(num) if valid_options.contains(&num) => return num,
            _ => println!("Please enter a valid option: {:?}", valid_options),
        }
    }
}

pub fn get_string_input(field: &str) -> String {
    println!("Enter a {}", field);
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Input could not be read");
    value.trim().to_string()
}

pub fn create_item() -> ToDoItem {
    let title = get_string_input("title");
    let description = get_string_input("description");

    ToDoItem::create(1, &title, &description)
}

pub fn main_loop() {

    let mut items: Vec<ToDoItem> = Vec::new();

    items.push(ToDoItem::create(1, "Learn Rust", "Learn to program the Rust programming language"));
    items.push(ToDoItem::create(2, "Learn Korean", "Learn to speak fluently in the Korean language"));
    

    loop {
        show_options();

        let valid_options = [1, 2, 3, 4];
        let user_input = get_option(&valid_options);
        
        match user_input {
            1 => print_todos(&items),
            2 => {
                let item = create_item();
                items.push(item);
            }
            4 => {
                println!("Goodbye");
                process::exit(0);
            }
            _ => println!("Great!"),
        }
    }
}