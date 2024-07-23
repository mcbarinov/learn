use bson::doc;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json_path::JsonPath;
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    #[serde_as(as = "bson::DateTime")]
    created_at: DateTime<Utc>,
    #[serde_as(as = "Option<bson::DateTime>")]
    updated_at: Option<DateTime<Utc>>,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost").await?;
    let coll: Collection<Data> = client.database("rust__mongodb").collection("data");
    coll.drop().await?;

    // insert new
    let new_data = Data { id: ObjectId::new(), name: "n1".to_string(), created_at: Utc::now(), updated_at: Some(Utc::now()) };
    let res = coll.insert_one(new_data).await.unwrap();
    println!("insert: {:?}", res);

    // find all
    let res: Vec<_> = coll.find(doc! {}).await.unwrap().try_collect().await.unwrap();
    dbg!(&res);

    // get as json
    let json_value = serde_json::to_value(res).unwrap();
    println!("{}", serde_json::to_string_pretty(&json_value).unwrap());

    // to human-readable
    println!("{}", serde_json::to_string_pretty(&human_readable_bson(json_value)).unwrap());

    Ok(())
}

fn human_readable_bson(mut value: Value) -> Value {
    let path = JsonPath::parse("$..['$date']").unwrap();
    let locations = path.query_located(&value).locations().map(|l| l.to_json_pointer()).collect::<Vec<_>>();
    for loc in locations {
        if let Some(node) = value.pointer_mut(&loc) {
            if let Some(date_map) = node.as_object() {
                if let Some(value) = date_map.get("$numberLong") {
                    if date_map.keys().len() == 1 && value.is_string() {
                        let value_str = value.as_str().unwrap();
                        if let Ok(value_int) = value_str.parse::<i64>() {
                            if let Some(dt) = DateTime::from_timestamp_millis(value_int) {
                                *node = dt.to_rfc3339().into();
                            }
                        };
                    }
                }
            }
        }
    }
    value
}
