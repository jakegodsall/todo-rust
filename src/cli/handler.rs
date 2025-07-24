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