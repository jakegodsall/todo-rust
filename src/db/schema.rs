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
