mod models;
mod cli;

use models::todo::ToDoItem;

fn main() {
    let mut items: Vec<ToDoItem> = Vec::new();
    // let mut id_counter: u32 = 3;

    items.push(ToDoItem::create(1, "Learn Rust", "Learn to program the Rust programming language"));
    items.push(ToDoItem::create(2, "Learn Korean", "Learn to speak fluently in the Korean language"));
    
    cli::handler::print_todos(&items);
}