use crate::models::todo::{ ToDoItem };
use std::fs::{ OpenOptions };
use std::error::Error;
use csv::{ Writer };

pub fn export(filename: &str, todos: &Vec<ToDoItem>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;

    let mut wtr = Writer::from_writer(file);

    for todo in todos {
        wtr.serialize(todo)?;
    }

    wtr.flush()?;
    Ok(())
}