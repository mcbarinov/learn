use serde::{Deserialize, Serialize};
use surrealdb::engine::local::RocksDb;
use surrealdb::opt::RecordId;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    id: Option<RecordId>,
    name: String,
    value: u64,
    tags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    let db = Surreal::new::<RocksDb>("tmp/db1").await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    let created: Vec<Data> = db
        .create("data")
        .content(Data {
            id: None,
            name: "d1".to_string(),
            value: 123,
            tags: vec!["a".to_string(), "b".to_string()],
        })
        .await?;
    dbg!(created);

    // find all records
    let all: Vec<Data> = db.select("data").await?;
    dbg!(all);

    Ok(())
}
