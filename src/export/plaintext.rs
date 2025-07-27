use crate::models::todo::{ ToDoItem };
use std::fs::{ File };
use std::io::{ Write, BufWriter };

pub fn export(filename: &str, todos: &Vec<ToDoItem>) {
    let file = File::create(filename).expect("Could not create file");
    let mut writer = BufWriter::new(file);

    for todo in todos {
        writeln!(writer, "{}", todo.string_repr())
            .expect("Failed to write to file");
    }
}