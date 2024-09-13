db.test.drop();
db.test.insertMany([
    {name: "n1", created_at: ISODate("2010-12-11T14:12:00Z")},
    {name: "n2", created_at: null},
    {name: "n3", created_at: ISODate("2014-12-11T14:12:00Z")}])
db.test.find({
    $or: [
        {created_at: null},
        {created_at: {$lt: ISODate("2012-12-11T14:12:00Z")}}
    ]
})

// {
//   "_id": ObjectId("5f55cf8ecadf1fe798cd71a0"),
//   "name": "n1",
//   "created_at": ISODate("2010-12-11T14:12:00Z")
// }
// {
//   "_id": ObjectId("5f55cf8ecadf1fe798cd71a1"),
//   "name": "n2",
//   "created_at": null
// }