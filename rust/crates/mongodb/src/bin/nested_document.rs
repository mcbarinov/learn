use bson::doc;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    nested: Option<NestedData>,
    #[serde_as(as = "bson::DateTime")]
    created_at: DateTime<Utc>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct NestedData {
    name: String,
    value: u8,
    #[serde_as(as = "bson::DateTime")]
    created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost").await?;
    let coll: Collection<Data> = client.database("rust__mongodb").collection("data");
    coll.drop().await?;

    let d1 = Data { id: ObjectId::new(), name: "d1".to_string(), nested: None, created_at: Default::default() };

    let res = coll.insert_one(d1).await?;
    dbg!(res);

    let nested = NestedData { name: "nested1".to_string(), value: 12, created_at: Default::default() };

    let nested = bson::to_document(&nested)?;

    let res = coll.update_one(doc! {"name": "d1"}, doc! {"$set": doc! {"nested": nested}}).await?;
    dbg!(res);

    let res = coll.find_one(doc! {"name": "d1"}).await?;
    dbg!(res);

    Ok(())
}
