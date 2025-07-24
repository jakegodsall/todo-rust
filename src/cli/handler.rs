use crate::models::todo::ToDoItem;

pub fn print_todos(todos: &Vec<ToDoItem>) {
    for todo in todos {
        println!("To Do #{}: {} - {}", todo.id, todo.title, todo.description);
    }
}