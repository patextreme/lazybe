GET {{ base_url }}/_system/reset
HTTP 200

POST {{ base_url }}/todos
{
  "title": "Task 1",
  "status": "Todo"
}
HTTP 201
[Captures]
task_id_1: jsonpath "$.id"

POST {{ base_url }}/todos
{
  "title": "Task 2",
  "status": "Todo"
}
HTTP 201

POST {{ base_url }}/todos
{
  "title": "Task 3",
  "status": "Todo"
}
HTTP 201

GET {{ base_url }}/todos
HTTP 200
[Asserts]
jsonpath "$" count == 3
jsonpath "$[0].title" == "Task 1"
jsonpath "$[1].title" == "Task 2"
jsonpath "$[2].title" == "Task 3"

