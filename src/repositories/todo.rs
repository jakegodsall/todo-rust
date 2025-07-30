use sqlite::{ Connection, State };
use chrono::{ Local };
use crate::models::ToDoItem;

struct ToDoRepository {
    conn: Connection,
}

impl ToDoRepository {
    fn insert(&self, todo: &ToDoItem) -> Result<i64> {
        let mut stmt = self
            .conn
            .prepare("INSERT INTO todos (title, description, created_at) VALUES (?, ?, ?);")?;

        stmt.bind((1, todo.title));
        stmt.bind((2, todo.description));
        stmt.bind((3, Local::now().to_str()));

        match stmt.next()? {
            State::Done => Ok(self.conn.last_insert_rowid()),
            Stade::Row => unreachable!("INSERT should not return rows"),
        }
    }
}