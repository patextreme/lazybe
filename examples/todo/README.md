# Overview

A minimal example with `Todo` and `Staff` entity.
A `Todo` must be assigned to some `Staff`.

# Running example

```bash
cargo r -p example-todo
```

# Running tests

__Prerequisites__
- [Hurl](https://hurl.dev/)

```bash
hurl --variables-file tests/env ./tests/*.hurl --test --jobs 1
```
