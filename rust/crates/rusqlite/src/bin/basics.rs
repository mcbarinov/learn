use chrono::{DateTime, Utc};
use rusqlite::{Connection, params, Result};

#[derive(Debug)]
struct Data {
    id: i64,
    name: String,
    value: i64,
    created_at: DateTime<Utc>,
}

fn create_data(conn: &Connection, data: &Data) -> Result<usize> {
    conn.execute(
        "INSERT INTO data (name, value, created_at) VALUES (?1, ?2, ?3)",
        params![data.name, data.value, data.created_at],
    )
}

fn create_multiple_data(conn: &Connection, data_vec: Vec<Data>) -> Result<()> {
    for data in data_vec {
        conn.execute(
            "INSERT INTO data (name, value, created_at) VALUES (?1, ?2, ?3)",
            params![data.name, data.value, data.created_at],
        )?;
    }
    Ok(())
}

fn read_data(conn: &Connection, id: i64) -> Result<Data> {
    conn.query_row("SELECT id, name, value, created_at FROM data WHERE id = ?1", params![id], |row| {
        Ok(Data { id: row.get(0)?, name: row.get(1)?, value: row.get(2)?, created_at: row.get(3)? })
    })
}

fn read_all_data(conn: &Connection) -> Result<Vec<Data>> {
    let mut stmt = conn.prepare("SELECT id, name, value, created_at FROM data")?;
    let data_iter =
        stmt.query_map([], |row| Ok(Data { id: row.get(0)?, name: row.get(1)?, value: row.get(2)?, created_at: row.get(3)? }))?;

    let mut data_vec = Vec::new();
    for data in data_iter {
        data_vec.push(data?);
    }
    Ok(data_vec)
}

fn update_data(conn: &Connection, id: i64, name: &str, value: i64) -> Result<usize> {
    conn.execute("UPDATE data SET name = ?1, value = ?2 WHERE id = ?3", params![name, value, id])
}

fn delete_data(conn: &Connection, id: i64) -> Result<usize> {
    conn.execute("DELETE FROM data WHERE id = ?1", params![id])
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE data (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            value INTEGER NOT NULL,
            created_at DATETIME NOT NULL
        )",
        [],
    )?;

    let data = Data { id: 0, name: "Sample Data".to_string(), value: 10, created_at: Utc::now() };

    create_data(&conn, &data)?;
    let read = read_data(&conn, 1)?;
    println!("Read: {:?}", read);

    update_data(&conn, 1, "Updated Data", 200)?;
    let updated = read_data(&conn, 1)?;
    println!("Updated: {:?}", updated);

    delete_data(&conn, 1)?;

    let data_vec = vec![
        Data { id: 0, name: "Sample Data 1".to_string(), value: 100, created_at: Utc::now() },
        Data { id: 0, name: "Sample Data 2".to_string(), value: 200, created_at: Utc::now() },
    ];

    create_multiple_data(&conn, data_vec)?;
    let all_data = read_all_data(&conn)?;
    for data in all_data {
        println!("Data: {:?}", data);
    }

    Ok(())
}
