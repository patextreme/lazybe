# Running example

```bash
cargo r -p example-todo
```

API documentation is availble at `http://localhost:8080`

# Running tests

__Prerequisites__
- [Hurl](https://hurl.dev/)

```bash
hurl --variables-file hurl/env ./hurl/*.hurl --test --jobs 1
```
