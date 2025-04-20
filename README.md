# Overview

LazyBE (lazy backend) is a collection of building blocks to quickly build a backend CRUD application.
It provides macros and trait impls that can be composed without having too much opinion on how to structure the application.

A typical backend application usually have boring parts where you just need to do basic CRUD and
fun parts where you do crazy stuff. LazyBE lets you skip the boring part and focus on the fun parts. 

## Features

- Derive data access layer from a struct
  - Using `sea-query` and `sqlx` under the hook which means you can use `postgres` and `sqlite` (no `mysql` just yet, but it should be trivial)
  - Automatically timestamp `created_at` and `updated_at` timestamp
  - See [dal example](./examples/kitchen-sink/examples/dal_minimal.rs)
- Derive `axum` endpoint from a struct
  - See [api example](./examples/kitchen-sink/examples/api_minimal.rs)
- Derive OpenAPI specification from a struct
  - See [todo example](./examples/todo)
- Custom validation
  - See [validation example](./examples/kitchen-sink/examples/api_validation.rs)
- Custom collection API (filter, sort, pagination)
  - See [collection API example](./examples/kitchen-sink/examples/api_pagination.rs)
- JSON field using `#[lazybe(json)]`

## A quick glance

A quick glance of LazyBE look something like this

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[lazybe(table = "book", endpoint = "/books", derive_to_schema)]
pub struct Book {
    #[lazybe(primary_key)]
    pub id: u32,
    pub title: String,
    pub author: String,
    pub publication_date: NaiveDate,
    #[lazybe(created_at)]
    pub created_at: DateTime<Utc>,
    #[lazybe(updated_at)]
    pub updated_at: DateTime<Utc>,
}
```

The `Entity` macro derives traits and sibling types in order to implement commonly used backend layers.

- The attribute `table = "book"` defines a table to perform CRUD operations.
- The optional attribute `endpoint = "/books"` defines a url path the resource should be exposed.
- The optional attribute `derive_to_schema` make sure `utoipa::ToSchema` is derived for sibling types.
- The attribute `#[lazybe(primary_key)]` defines a primary key so you can get the book by its ID.
- The attribute `#[lazybe(created_at)]` and `#[lazybe(updated_at)]` automatically timestamp when an instance is created or updated.

With this macro, you have the following backend layers are implemented

1. Data access layer (`sqlx`, `sea-query`)
2. API layer (`axum`, `serde`)
3. OpenAPI specification (`utoipa`)

Then, the `Book` can be exposed on REST API using `axum` like this

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ctx = SqliteDbCtx;
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    let openapi = OpenApiBuilder::new()
        .info(Info::new("Todo Example", "0.1.0"))
        .servers(Some([Server::new("http://localhost:8080")]))
        .build()
        .merge_from(Book::get_endpoint_doc(None))
        .merge_from(Book::list_endpoint_doc(None))
        .merge_from(Book::create_endpoint_doc(None))
        .merge_from(Book::update_endpoint_doc(None))
        .merge_from(Book::replace_endpoint_doc(None))
        .merge_from(Book::delete_endpoint_doc(None));

    let app = Router::new()
        .merge(Redoc::with_url("/", openapi))
        .merge(Book::get_endpoint())
        .merge(Book::list_endpoint())
        .merge(Book::create_endpoint())
        .merge(Book::update_endpoint())
        .merge(Book::replace_endpoint())
        .merge(Book::delete_endpoint())
        .with_state(AppState { ctx, pool });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server is listening on 0.0.0.0:8080");
    lazybe::axum::serve(listener, app).await?;
    Ok(())
}
```

This would create the following endpoints for `Book` resource
- `POST /books` - Create a new book and save it in `book` table.
- `GET /books` - Get a collection of books
- `GET /books/{id}` - Get a book by its ID
- `PUT /books/{id}` - Replace an existing book
- `PATCH /books/{id}` - Partial update an existing book
- `DELETE /books/{id}` - Delete a book by its ID

And also create OpenAPI specification for `Book` resource and serve it using Redoc UI.

![](./docs/redoc.png)

Of course this is a bit hand wavy, for a fully working example
please check [minimal API example](./examples/kitchen-sink/examples/api_minimal.rs).

# Documentation

The only doc right now is the [example directory](./examples). There is no fancy doc site just yet.
