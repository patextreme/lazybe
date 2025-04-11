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
jsonpath "$.page" == 1
jsonpath "$.page_size" == 10
jsonpath "$.total_records" == 3
jsonpath "$.total_pages" == 1
jsonpath "$.data" count == 3

GET {{ base_url }}/todos
[Query]
size: 2
HTTP 200
[Asserts]
jsonpath "$.page" == 1
jsonpath "$.page_size" == 2
jsonpath "$.total_records" == 3
jsonpath "$.total_pages" == 2
jsonpath "$.data" count == 2

GET {{ base_url }}/todos
[Query]
page: 0
size: 2
HTTP 200
[Asserts]
jsonpath "$.page" == 1
jsonpath "$.page_size" == 2
jsonpath "$.total_records" == 3
jsonpath "$.total_pages" == 2
jsonpath "$.data" count == 2

GET {{ base_url }}/todos
[Query]
page: 1
size: 2
HTTP 200
[Asserts]
jsonpath "$.page" == 1
jsonpath "$.page_size" == 2
jsonpath "$.total_records" == 3
jsonpath "$.total_pages" == 2
jsonpath "$.data" count == 2

GET {{ base_url }}/todos
[Query]
page: 2
size: 2
HTTP 200
[Asserts]
jsonpath "$.page" == 2
jsonpath "$.page_size" == 2
jsonpath "$.total_records" == 3
jsonpath "$.total_pages" == 2
jsonpath "$.data" count == 1

#######################################
# List tasks with query params
#######################################
# All tasks
GET {{ base_url }}/todos
[Query]
status: Todo
HTTP 200
[Asserts]
jsonpath "$.page" == 1
jsonpath "$.page_size" == 10
jsonpath "$.total_records" == 3
jsonpath "$.total_pages" == 1
jsonpath "$.data" count == 3

# Alice picks up a task
PATCH {{ base_url }}/todos/{{ task_id_1 }}
{
  "status": "Doing"
}
HTTP 200

# All task with 'Todo' status
GET {{ base_url }}/todos
[Query]
status: Todo
HTTP 200
[Asserts]
jsonpath "$.page" == 1
jsonpath "$.page_size" == 10
jsonpath "$.total_records" == 2
jsonpath "$.total_pages" == 1
jsonpath "$.data" count == 2

# All task with 'Doing' status
GET {{ base_url }}/todos
[Query]
status: Doing
HTTP 200
[Asserts]
jsonpath "$.page" == 1
jsonpath "$.page_size" == 10
jsonpath "$.total_records" == 1
jsonpath "$.total_pages" == 1
jsonpath "$.data" count == 1

# All task with 'Todo' status assigned to Alice
GET {{ base_url }}/todos
[Query]
status: Todo
assignee: {{ alice_id }}
HTTP 200
[Asserts]
jsonpath "$.page" == 1
jsonpath "$.page_size" == 10
jsonpath "$.total_records" == 1
jsonpath "$.total_pages" == 1
jsonpath "$.data" count == 1
