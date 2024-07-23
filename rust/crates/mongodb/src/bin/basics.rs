use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Result;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    tags: Vec<String>,
    value: i64,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::with_uri_str("mongodb://localhost").await?;
    let coll: Collection<Data> = client.database("rust__mongodb").collection("data");

    // drop collection data first
    coll.drop().await?;

    // insert new
    let new_data = Data {
        id: ObjectId::new(),
        name: "n1".into(),
        tags: vec!["abc".into(), "def".into()],
        value: 123,
        created_at: Utc::now(),
    };
    let res = coll.insert_one(new_data).await?;
    dbg!(res);

    let new_data = Data {
        id: ObjectId::new(),
        name: "n2".into(),
        tags: vec!["abc2".into(), "def2".into()],
        value: 1232,
        created_at: Utc::now(),
    };
    let res = coll.insert_one(new_data).await?;
    dbg!(res);

    // find one
    let res = coll.find_one(doc! {"name": "n1"}).await?;
    println!("find_one: {:?}", res);

    // find many
    // Install `futures` package for TryStreamExt. It's for .try_collect() method
    let res: Vec<_> = coll.find(doc! {}).sort(doc! {"name": -1}).await?.try_collect().await?;
    println!("find_many: {:?}", res);

    Ok(())
}
