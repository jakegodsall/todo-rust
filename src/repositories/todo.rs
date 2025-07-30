use sqlite::{ Connection, State, Error };
use chrono::{ DateTime, Local, NaiveDateTime, TimeZone };
use crate::models::todo::ToDoItem;

pub struct ToDoRepository {
    conn: Connection,
}

impl ToDoRepository {
    pub fn open(db: &str) -> Self {
        Self {
            conn: sqlite::open(db).unwrap()
        }
    }
    
    pub fn get_all(&self) -> Result<Vec<ToDoItem>> {
        let mut stmt = self.conn.prepare(
                    "SELECT id, title, description, created_at FROM todos ORDER BY id ASC",
                )?;

        let mut items = Vec::new();

        while let State::Row = stmt.next()? {
            let created_at_str: String = stmt.read(3)?;
            let naive = NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")?;
            let created_at = Local
                .from_local_datetime(&naive)
                .single()?;

            let completed_at_str: Option<String> = stmt.read(4)?;
            
            
            let item = ToDoItem {
                id: stmt.read::<i64, usize>(0)?,
                title: stmt.read::<String, usize>(1)?,
                description: stmt.read::<String, usize>(2)?,
                created_at: created_at,
            }

            items.push(item);
        }

        Ok(items)
    }

    pub fn create(&self, todo: &ToDoItem) -> Result<i64> {
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