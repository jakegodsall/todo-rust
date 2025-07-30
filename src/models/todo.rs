use chrono::{ DateTime, Local };
use serde::{ Serialize };

#[derive(Serialize)]
pub struct ToDoItem {
    pub id: u32,
    pub title: String,
    pub description: String,

    #[serde(with = "datetime_format")]
    pub created_at: DateTime<Local>,

    #[serde(serialize_with = "datetime_format::serialize_option")]
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

    pub fn complete(&mut self) {
        self.completed_at = Some(Local::now())
    }

    pub fn is_complete(&self) -> bool {
        self.completed_at.is_some()
    }
}

mod datetime_format {
    use chrono::{ DateTime, Local };
    use serde::{ self, Serializer };

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(
        date: &DateTime<Local>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn serialize_option<S> (
        date: &Option<DateTime<Local>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => serialize(dt, serializer),
            None => serializer.serialize_none(),
        }
    }
}