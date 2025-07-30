use std::io;
use std::fs;

pub fn check_db_exists() -> io::Result<bool> {
    let pattern = ".db";

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.ends_with(pattern) {
                return Ok(true)
            }
        }
    }

    return Ok(false)
}

pub fn define() {
    let connection = sqlite::open("test.db")
        .unwrap();

    let query = "
        CREATE TABLE todo (
            title TEXT,
            description TEXT,
            created_at TEXT,
            completed_at TEXT
        );
    ";

    connection.execute(query).unwrap();
}
