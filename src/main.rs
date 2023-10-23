use rusqlite::Connection;

fn main() {
    let conn = Connection::open("my_database.db").expect("Failed to open database");
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    )
    .expect("Failed to create table");
}
