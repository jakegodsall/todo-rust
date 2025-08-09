use crate::utils::db_time::{ DB_DATETIME_FMT };
use sqlite::{ Connection, State };
use chrono::{ Local, NaiveDate, NaiveDateTime, TimeZone, DateTime };
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
    
    pub fn get_all(&self) -> Result<Vec<ToDoItem>, Box<dyn std::error::Error>> {
        let mut stmt = self.conn.prepare(
                    "SELECT id, title, description, created_at FROM todos ORDER BY id ASC",
                )?;

        let mut items = Vec::new();

        while let State::Row = stmt.next()? {
            let created_at_str: String = stmt.read(3)?;
            let naive = NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")?;
            let created_at = Local
                .from_local_datetime(&naive)
                .single()
                .ok_or(|e| panic!("{}", e));

            let completed_at_str: Option<String> = stmt.read(4)?;
            let completed_at = Option<DateTime<Local>> = completed_at_str
                .as_deref()
                .map(|s| -> Result<DateTime<Local>, chrono::ParseError> {
                    let date = NaiveDateTime::parse_from_str(s, "%Y-%m-%d");
                    let naive = date.and_hms_opt(0, 0, 0).unwrap();
                    let local = Local.from_local_datetime(&naive)
                        .single()                              // exact match
                        .or_else(|| Local.from_local_datetime(&naive).earliest()) // if ambiguous (DST)
                        .or_else(|| Local.from_local_datetime(&naive).latest())   // fallback
                        .expect("nonexistent local time");
                    Ok(local)
                })
                .transpose()?;

            let item = ToDoItem {
                id: stmt.read::<i64, usize>(0)?,
                title: stmt.read::<String, usize>(1)?,
                description: stmt.read::<String, usize>(2)?,
                created_at: created_at,
                completed_at: completed_at
            };

            items.push(item);
        }

        Ok(items)
    }

    pub fn create(&self, todo: &ToDoItem) -> Result<i64, Box<dyn std::error::Error>> {
        let mut stmt = self
            .conn
            .prepare("INSERT INTO todos (title, description, created_at) VALUES (?, ?, ?);")?;

        stmt.bind((1, todo.title.as_str()));
        stmt.bind((2, todo.description.as_str()));
        let now_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let now_str: &str = &now_string;
        stmt.bind((3, now_str));

        match stmt.next()? {
            State::Done => Ok(1),
            State::Row => unreachable!("INSERT should not return rows"),
        }
    }

    pub fn get_by_id(&self, id: i64) -> Result<ToDoItem, Box<dyn std::error::Error>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, description, created_at, completed_at FROM todos WHERE id = ?")?;

        stmt.bind((1, id));

        if let State::Row = stmt.next()? {
                let created_at_str: String = stmt.read(3)?;
                let naive_created_at = NaiveDateTime::parse_from_str(created_at_str.as_str(), DB_DATETIME_FMT)?;
                let created_at = Local
                    .from_local_datetime(&naive_created_at)
                    .single()
                    .or_else(|| Local.from_local_datetime(&naive_created_at).earliest())
                    .or_else(|| Local.from_local_datetime(&naive_created_at).latest())
                    .ok_or("nonexistent local time")?;

                let completed_at_str: Option<String> = stmt.read(4)?;
                let completed_at: Option<DateTime<Local>> = match completed_at_str.as_deref() {
                    Some(s) => {
                        let d = NaiveDate::parse_from_str(&s, "%Y-%m-%d")?;
                        let naive_midnight = d.and_hms_opt(0, 0, 0).ok_or("invalid midnight time")?;
                        Some(
                            Local
                                .from_local_datetime(&naive_midnight)
                                .single()
                                .or_else(|| Local.from_local_datetime(&naive_midnight).earliest())
                                .or_else(|| Local.from_local_datetime(&naive_midnight).latest())
                                .ok_or("nonexistent local time")?,
                        )
                    }
                    None => None,
                };

                Ok(ToDoItem {
                    id: stmt.read::<i64, _>(0)?,
                    title: stmt.read::<String, _>(1)?,
                    description: stmt.read::<String, _>(2)?,
                    created_at,
                    completed_at,
                })
        } else {
            Err("todo not found".into())
        }
    }   

    pub fn complete_by_id(&self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let mut stmt = self
            .conn
            .prepare("UPDATE todos SET completed_at = ? WHERE id = ?")?;

        let now_str = Local::now().format(DB_DATETIME_FMT).to_string();
        stmt.bind((1, now_str.as_str()));
        stmt.bind((2, id));

        
    }
}