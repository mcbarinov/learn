const schema = {
    $jsonSchema: {
        required: ["name", "value"]
    }
}
db.test.drop()
db.createCollection("test", {validator: schema})
db.test.insertOne({"name": "n1"})