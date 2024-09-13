const schema = {
    $jsonSchema: {
        required: ["name", "status", "value", "created_at", "updated_at"],
        additionalProperties: false,
        properties: {
            _id: {bsonType: "objectId"},
            name: {bsonType: "string"},
            status: {enum: ["OK", "ERROR", "UNKNOWN"]},
            value: {bsonType: "int"},
            created_at: {bsonType: "date"},
            updated_at: {bsonType: ["date", "null"]}
        }
    }
}
db.test.drop()
db.createCollection("test", {validator: schema})
db.test.insertOne({name: "n1", status: "OK", value: 1, created_at: new Date(), updated_at: null})