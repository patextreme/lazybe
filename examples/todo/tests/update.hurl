GET {{ base_url }}/_system/reset
HTTP 200

POST {{ base_url }}/staffs
{
  "name": "Alice"
}
HTTP 201
[Captures]
staff_id: jsonpath "$.id"

GET {{ base_url }}/staffs/{{ staff_id }}
HTTP 200
[Asserts]
jsonpath "$.name" == "Alice"
[Captures]
created_at: jsonpath "$.created_at"
updated_at: jsonpath "$.updated_at"

PATCH {{ base_url }}/staffs/{{ staff_id }}
[Options]
delay: 1s
{
  "name": "Bob"
}
HTTP 200
[Asserts]
jsonpath "$.name" == "Bob"
jsonpath "$.created_at" == "{{ created_at }}"
jsonpath "$.updated_at" != "{{ updated_at }}"
[Captures]
updated_at_2: jsonpath "$.updated_at"

PATCH {{ base_url }}/staffs/{{ staff_id }}
[Options]
delay: 1s
{
  "name": "Charlie"
}
HTTP 200
[Asserts]
jsonpath "$.name" == "Charlie"
jsonpath "$.created_at" == "{{ created_at }}"
jsonpath "$.updated_at" != "{{ updated_at_2 }}"

