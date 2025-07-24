mod models;
mod cli;

use std::io;

use models::todo::ToDoItem;

fn main() {
    let mut items: Vec<ToDoItem> = Vec::new();
    let mut id_counter: u32 = 3;

    items.push(ToDoItem::create(1, "Learn Rust", "Learn to program the Rust programming language"));
    items.push(ToDoItem::create(2, "Learn Korean", "Learn to speak fluently in the Korean language"));
    
    loop {
        // println!("Enter a title: ");
        // let mut title = String::new();
        // io::stdin().read_line(&mut title).expect("Failed to read line");
        // let title = title.trim().to_string();

        // println!("Enter a description: ");
        // let mut description = String::new();
        // io::stdin().read_line(&mut description).expect("Failed to read line");
        // let description = description.trim().to_string();

        // let item: ToDoItem = ToDoItem::create(id_counter, &title, &description);
        // id_counter += 1;

        // items.push(item);
        break

    }
    cli::handler::print_todos(&items);
}