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
    #[serde_as(as = "bson::DateTime")]
    created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost").await?;
    let coll: Collection<Data> = client.database("rust__mongodb").collection("data");
    coll.drop().await?;

    Ok(())
}
