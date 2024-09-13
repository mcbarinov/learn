db.test.drop();
db.test.insertMany([
    {name: "n1", created_at: ISODate("2020-12-11T14:12:00Z")},
    {name: "n2", created_at: null},
    {name: "n3", created_at: ISODate("2014-12-11T14:12:00Z")},
    {name: "n4", created_at: null},
    {name: "n6", created_at: ISODate("2016-12-11T14:12:00Z")}
]);
db.test.find({"$or": [{"created_at": null}, {"created_at": {"$lt": ISODate("2022-12-11T14:12:00Z")}}]}).sort({"checked_at": 1});

// It doesn't work!!! Do two queries. First query where checked_at = None

// {
//   "_id": ObjectId("5f8d3da56ca36741eb5babe8"),
//   "name": "n1",
//   "created_at": ISODate("2020-12-11T14:12:00Z")
// }
// {
//   "_id": ObjectId("5f8d3da56ca36741eb5babe9"),
//   "name": "n2",
//   "created_at": null
// }
// {
//   "_id": ObjectId("5f8d3da56ca36741eb5babea"),
//   "name": "n3",
//   "created_at": ISODate("2014-12-11T14:12:00Z")
// }
// {
//   "_id": ObjectId("5f8d3da56ca36741eb5babeb"),
//   "name": "n4",
//   "created_at": null
// }
// {
//   "_id": ObjectId("5f8d3da56ca36741eb5babec"),
//   "name": "n6",
//   "created_at": ISODate("2016-12-11T14:12:00Z")
// }