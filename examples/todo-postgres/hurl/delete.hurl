GET {{ base_url }}/_system/reset
HTTP 200

POST {{ base_url }}/todos
{
  "title": "Do homework",
  "description": "due this week",
  "status": "Todo"
}
HTTP 201
[Captures]
task_id: jsonpath "$.id"

GET {{ base_url }}/todos/{{ task_id }}
HTTP 200

DELETE {{ base_url }}/todos/{{ task_id }}
HTTP 200

GET {{ base_url }}/todos/{{ task_id }}
HTTP 404
