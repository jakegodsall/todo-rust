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

pub fn show_options(options: &Vec<String>) {


    println!("----- WELCOME -----");
    println!("Select an option: ");
    for (idx, option) in options.iter().enumerate() {
        println!("{}. {}", idx+1, option);
    }
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

pub fn complete_todo(todos: &mut Vec<ToDoItem>) {
    if todos.is_empty() {
        println!("No todos to complete.");
        return;
    }

    println!("Select a todo to complete: ");
    print_todos(todos);

    let valid_options: Vec<i32> = (1..=todos.len() as i32).collect();
    let selected = get_option(&valid_options);

    let index = (selected - 1) as usize;

    todos[index].complete();
    print_todos(todos);
}

pub fn delete_todo(todos: &mut Vec<ToDoItem>) {
    if todos.is_empty() {
        println!("No todos to delete");
        return;
    }

    println!("Select a todo to delete: ");
    print_todos(todos);

    let valid_options: Vec<i32> = (1..=todos.len() as i32).collect();
    let selected = get_option(&valid_options);

    let index = (selected - 1) as usize;

    todos.remove(index);
    print_todos(todos);
}

pub fn main_loop() {
    let options: Vec<String> = vec![
        String::from("View todo items"),
        String::from("Add todo item"),
        String::from("Check off a todo item"),
        String::from("Delete a todo item"),
        String::from("Quit"),
    ];

    let mut items: Vec<ToDoItem> = Vec::new();
    let mut id_counter: u32 = 3;

    items.push(ToDoItem::create(1, "Learn Rust", "Learn to program the Rust programming language"));
    items.push(ToDoItem::create(2, "Learn Korean", "Learn to speak fluently in the Korean language"));
    
    loop {
        show_options(&options);

        let valid_options: Vec<i32> = (1..=options.len() as i32).collect();
        let user_input = get_option(&valid_options);
        
        match user_input {
            1 => print_todos(&items),
            2 => {
                let title = get_string_input("title");
                let description = get_string_input("description");
                let item = ToDoItem::create(id_counter, &title, &description);
                id_counter += 1;
                items.push(item);
            },
            3 => complete_todo(&mut items),
            4 => delete_todo(&mut items),
            5 => {
                println!("Goodbye");
                process::exit(0);
            }
            _ => println!("Great!"),
        }
    }
}