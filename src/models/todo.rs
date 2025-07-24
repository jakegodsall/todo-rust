use chrono::{ DateTime, Local };

pub struct ToDoItem {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>
}

impl ToDoItem {
    pub fn create(id: u32, title: &str, description: &str) -> Self {
        ToDoItem {
            id,
            title: title.to_string(),
            description: description.to_string(),
            created_at: Local::now(),
            completed_at: None,
        }
    }

    pub fn string_repr(&self) -> String {
        format!("To Do #{}: {} - {}", self.id, self.title, self.description)
    }
}