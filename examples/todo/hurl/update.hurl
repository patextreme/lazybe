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
[Captures]
created_at: jsonpath "$.created_at"
updated_at: jsonpath "$.updated_at"

PATCH {{ base_url }}/todos/{{ task_id }}
[Options]
delay: 1s
{
  "status": "Doing"
}
HTTP 200
[Asserts]
jsonpath "$.title" == "Do homework"
jsonpath "$.description" == "due this week"
jsonpath "$.status" == "Doing"
jsonpath "$.created_at" == "{{ created_at }}"
jsonpath "$.updated_at" != "{{ updated_at }}"
[Captures]
updated_at_2: jsonpath "$.updated_at"

PUT {{ base_url }}/todos/{{ task_id }}
[Options]
delay: 1s
{
  "title": "Do math homework",
  "description": null,
  "status": "Done"
}
HTTP 200
[Asserts]
jsonpath "$.title" == "Do math homework"
jsonpath "$.description" == null
jsonpath "$.status" == "Done"
jsonpath "$.created_at" == "{{ created_at }}"
jsonpath "$.updated_at" != "{{ updated_at }}"
jsonpath "$.updated_at" != "{{ updated_at_2 }}"

