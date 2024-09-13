db.test.drop()
db.test.insertMany([
    { name: "n1", interval: 10, checked_at: new ISODate("2019-10-10 12:12:12") },
    { name: "n2", interval: 20, checked_at: new ISODate("2020-10-10 12:12:12") },
    { name: "n3", interval: 30, checked_at: new ISODate("2020-10-10 12:12:12") }])

db.test.aggregate([
    {
        $match: {
            $expr: {
                $lt: ["$checked_at", { $subtract: ["$$NOW", { $multiply: ["$interval", 1000] }] }]
            }
        }
    }
])
