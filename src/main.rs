use sqlite::{Connection, OpenFlags, State};

fn main() {
    // Open a connection, create a table, and insert rows:
    let connection = Connection::open_with_flags(":memory:", OpenFlags::READ_WRITE | OpenFlags::CREATE)
        .unwrap();

    let query = "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Alice', 42);
        INSERT INTO users VALUES ('Bob', 69);
    ";
    connection.execute(query).unwrap();

    // Select and process rows using an iterator:
    let query = "SELECT * FROM users WHERE age > 50";

    connection
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                println!("{} = {}", name, value.unwrap());
            }
            true
        })
        .unwrap();

    // Run the same query using a prepared statement:
    let query = "SELECT * FROM users WHERE age > ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind(1).unwrap().bind(50).unwrap();

    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String>("name").unwrap());
        println!("age = {}", statement.read::<i64>("age").unwrap());
    }

    // Run the same query using a cursor:
    let cursor = connection.prepare(query).unwrap().cursor().unwrap();
    cursor.bind(1, 50).unwrap();

    for row in cursor {
        let row = row.unwrap();
        println!("name = {}", row.read::<String>("name").unwrap());
        println!("age = {}", row.read::<i64>("age").unwrap());
    }
}


