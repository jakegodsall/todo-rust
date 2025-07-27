use crate::models::todo::{ ToDoItem };
use std::fs::File;

fn export(filename: &str, todos: &Vec<ToDoItem>) {
    let file = File::open(filename);
    let mut writer = BufWriter::new(file);

    for todo in todos {
        writeln!(writer, "{}", todo.string_repr());
    }

    Ok(())
}