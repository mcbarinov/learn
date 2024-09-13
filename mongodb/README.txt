--> Enable auth
1) Turn off auth in mongod.conf
security:
  authorization: disable
2) Add user
use admin
db.createUser(
  {
    user: "admin",
    pwd: "abc123",
    roles: [ { role: "userAdminAnyDatabase", db: "admin" }, "readWriteAnyDatabase" ]
  }
)
3) Update mongod.conf and restart mongod
security:
  authorization: enabled

systemctl restart mongod

4) In app use the connection string
mongodb://admin:abc1231@localhost/my_app?authSource=admin


--> Check the current number of connections
db.serverStatus().connections

--> We can't update _id field. We can only create a new object and delete an old.

--> Restore to docker
docker run -it --rm --name mongorestore --net ${NET} -v ${DUMPDIR}:/var/dump --link ${CONTAINERNAME}:${CONTAINERNAME} mongo mongorestore --host ${CONTAINERNAME} /var/dump

--> Backup
mongodump -u <username> -p <password> -d <database> --gzip --archive=outfile.gz
mongodump -u <username> -p <password> -d <database> --authenticationDatabase <auth_database> --gzip --archive=outfile.gz

--> Restore
mongorestore --drop --gzip --archive=archive.gz

--> Limit mongodb cache size. Othewise mongo will take half of all memory on the server
mongod --wiredTigerEngineConfigString="cache_size=500M"


--> List all indexes on a collection
db.people.getIndexes()

--> Rename field
db.getCollection('xxx').updateMany({}, {$rename: {"oldName": "newName"}});

--> Delete field
db.getCollection('xxx').updateMany({}, {$unset: {"field1": ""}});

--> Explain query (check usage of indexes)
db.getCollection('datas').find().sort({createdAt: -1}).limit(10).explain("executionStats")

--> Check slow (> 100 ms) queries
tail -f /var/log/mongodb/mongod.log

--> mongotop (like top)
mongotop -u admin -p ************ --authenticationDatabase=admin

--> Get connection info
db.getCollectionInfos({name: "system_messages"})

--> Drop db from shell
mongo test__rocket_mongodb --eval "db.dropDatabase()"

--> Rename database
mongodump --archive="old_name_dump.db" --db=old_name
mongorestore --archive="old_name_dump.db" --nsFrom="old_name.*" --nsTo="new_name.*"
drop old database
