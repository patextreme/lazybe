GET {{ base_url }}/_system/reset
HTTP 200

#######################################
# Create staff and assign tasks
#######################################
POST {{ base_url }}/staffs
{
  "name": "Alice"
}
HTTP 201
[Captures]
alice_id: jsonpath "$.id"

POST {{ base_url }}/staffs
{
  "name": "Bob"
}
HTTP 201
[Captures]
bob_id: jsonpath "$.id"

POST {{ base_url }}/todos
{
  "title": "Task 1",
  "assignee": {{ alice_id }},
  "status": "Todo"
}
HTTP 201
[Captures]
task_id_1: jsonpath "$.id"

POST {{ base_url }}/todos
{
  "title": "Task 2",
  "assignee": {{ alice_id }},
  "status": "Todo"
}
HTTP 201

POST {{ base_url }}/todos
{
  "title": "Task 3",
  "assignee": {{ bob_id }},
  "status": "Todo"
}
HTTP 201

#######################################
# List staffs
#######################################
GET {{ base_url }}/staffs
HTTP 200
[Asserts]
jsonpath "$" count == 2

#######################################
# List tasks
#######################################
GET {{ base_url }}/todos
HTTP 200
[Asserts]
jsonpath "$" count == 3
