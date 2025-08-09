use std::{io, process};

use crate::models::todo::ToDoItem;
use crate::export::plaintext::export as plaintext_export;
use crate::export::csv::export as csv_export;
use crate::repositories::todo::{ ToDoRepository };

pub fn print_todos(todos: &Vec<ToDoItem>) {
    println!("----- TODOS -----");
    for todo in todos.iter() {
        println!("{}: {}", todo.string_repr(), checkbox(todo.is_complete()));
    }
    println!("\n");
}

pub fn checkbox(condition: bool) -> &'static str {
    if condition {
        "☑"
    } else {
        "☐"
    }
}

pub fn show_options(options: &Vec<String>) {
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

pub fn complete_todo() {
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

pub fn export(todos: &Vec<ToDoItem>) -> bool {
    let options: Vec<String> = vec![
        String::from("Plaintext"),
        String::from("CSV"),
    ];

    show_options(&options);
    let valid_options: Vec<i32> = (1..=options.len() as i32).collect();
    let user_input = get_option(&valid_options);

    match user_input {
        1 => {
            plaintext_export("todos.txt", &todos)
                .expect("Failed");
            true
            },
        2 => {
            csv_export("todos.csv", &todos)
                .expect("Failed");
            true
            },
        _ => false
    }
}

pub fn main_loop() {
    let options: Vec<String> = vec![
        String::from("View todo items"),
        String::from("Add todo item"),
        String::from("Check off a todo item"),
        String::from("Delete a todo item"),
        String::from("Export"),
        String::from("Quit"),
    ];

    let todo_repository = ToDoRepository::open("test.db");

    println!("----- WELCOME -----");

    loop {
        show_options(&options);

        let valid_options: Vec<i32> = (1..=options.len() as i32).collect();
        let user_input = get_option(&valid_options);
        
        match user_input {
            1 => {
                let items = todo_repository.get_all().unwrap();
                print_todos(&items);
            },
            2 => {
                let title = get_string_input("title");
                let description = get_string_input("description");
                let todo = ToDoItem::create(5, &title, &description);
                todo_repository.create(&todo);
            },
            3 => complete_todo(&mut items),
            4 => delete_todo(&mut items),
            5 => {
                export(&items);
            },
            6 => {
                println!("Goodbye");
                process::exit(0);
            }
            _ => println!("Great!"),
        }
    }
}