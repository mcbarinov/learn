db.test.drop()
db.test.insertMany([{name: "n1"}, {name: "n1"}, {name: "n2"}])
db.test.distinct("name")
// [ "n1", "n2" ]