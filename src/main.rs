mod models;

use std::io;

use models::todo::ToDoItem;

fn main() {
    let mut id_counter: u32 = 1;
    let mut items: Vec<ToDoItem> = Vec::new();
    
    loop {
        println!("Enter a title: ");
        let mut title = String::new();
        io::stdin().read_line(&mut title).expect("Failed to read line");
        let title = title.trim().to_string();

        println!("Enter a description: ");
        let mut description = String::new();
        io::stdin().read_line(&mut description).expect("Failed to read line");
        let description = description.trim().to_string();

        let item: ToDoItem = ToDoItem::create(id_counter, &title, &description);
        id_counter += 1;

        println!("Added item: #{}: {} - {} (Created: {})", item.id, item.title, item.description, item.created_at);
        items.push(item);
    }
}