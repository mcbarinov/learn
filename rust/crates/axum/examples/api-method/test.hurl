GET http://localhost:3000/api-post/data/77/handler1
HTTP 200
[Asserts]
body contains "handler1: 77"

GET http://localhost:3000/api-delete/data/77/handler2
HTTP 200
[Asserts]
jsonpath "$.response" == "77"