use crate::models::todo::{ ToDoItem };
use std::fs::{ OpenOptions };
use std::io::{ Write, BufWriter };

pub fn export(filename: &str, todos: &Vec<ToDoItem>) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&filename)?;

    let mut writer = BufWriter::new(file);

    for todo in todos {
        writeln!(writer, "{}", todo.string_repr())
            .expect("Failed to write to file");
    }

    Ok(())
}