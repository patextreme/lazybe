//! A building block for quickly (and lazily) creating CRUD backend applications.
//!
//! When building a backend application in Rust, you'll often need to expose resources
//! via an HTTP API, optionally perform validations, and interact with a database.
//! Much of this work tends to be boilerplate, with little domain logic or custom code.
//!
//! If you are already using:
//! - [`axum`] for HTTP interface
//! - [`sqlx`] for database interactions
//! - [`utoipa`] for OpenAPI docuementation
//!
//! LazyBE provides building blocks that implements traits from these crates
//! allowing you to quickly assemble components for your application.
//! It lets you skip the mundane parts and work on the fun parts where you get to do crazy stuff!
//!
//!
//! # Usage
//!
//! ## Entity
//!
//! The core concept in LazyBE is the [`Entity`], which represents an identifiable resource within the domain.
//! For example, in a Todo application, the Todo itself is an entity.
//!
//! ```
//! use chrono::{DateTime, Utc};
//! use lazybe::macros::Entity;
//!
//! #[derive(Entity)]
//! #[lazybe(table = "todo")]
//! pub struct Todo {
//!     #[lazybe(primary_key)]
//!     pub id: i32,
//!     pub title: String,
//!     pub description: Option<String>,
//!     pub is_completed: bool,
//!     #[lazybe(created_at)]
//!     pub created_at: DateTime<Utc>,
//!     #[lazybe(updated_at)]
//!     pub updated_at: DateTime<Utc>,
//! }
//! ```
//!
//! The [`Entity`](macro@macros::Entity) macro derives the necessary traits and sibling types for you.
//! For the Todo example, these implementations are automatically generated:
//!
//! - Trait impls
//!   - [`Entity`] - definition of the entity
//!   - [`TableEntity`] - additional definition of entity that can be read from / written to the database
//!   - CRUD database queries
//!     - [`GetQuery`](query::GetQuery)
//!     - [`ListQuery`](query::ListQuery)
//!     - [`CreateQuery`](query::CreateQuery)
//!     - [`UpdateQuery`](query::UpdateQuery)
//!     - [`DeleteQuery`](query::DeleteQuery)
//!   - Entity CRUD operations
//!     - [`GetEntity`](entity::ops::GetEntity)
//!     - [`ListEntity`](entity::ops::ListEntity)
//!     - [`CreateEntity`](entity::ops::CreateEntity)
//!     - [`UpdateEntity`](entity::ops::UpdateEntity)
//!     - [`DeleteEntity`](entity::ops::DeleteEntity)
//! - Sibling types
//!   - `CreateTodo` - a sibling type used for creating a new instance
//!   - `UpdateTodo` - a sibling type used for updating an instance
//!   - `ReplaceTodo` - a sibling type used for replacing an instance
//!   - `TodoFilter` - a sibling type for creating a [`Filter`](filter::Filter)
//!   - `TodoSort` - a sibling type for creating a [`Sort`](sort::Sort)
//!   - `TodoSqlxRow` - a sibling type used by [`sqlx`] that implements [`sqlx::FromRow`]
//!   - `TodoSeaQueryIdent` - a sibling type used by [`sea_query`] for buiding query
//!
//!
//! ## Newtype and Enum
//!
//! It is possible to use Enum and Newtype pattern on your Entity.
//!
//! ```
//! use chrono::{DateTime, Utc};
//! use lazybe::macros::{Entity, Enum, Newtype};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Clone, Newtype)]
//! pub struct TodoId(i32);
//!
//! #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Enum)]
//! pub enum Status {
//!     Todo,
//!     Doing,
//!     Done,
//! }
//!
//! #[derive(Entity)]
//! #[lazybe(table = "todo")]
//! pub struct Todo {
//!     #[lazybe(primary_key)]
//!     pub id: TodoId,
//!     pub title: String,
//!     pub description: Option<String>,
//!     pub status: Status,
//! }
//! ```
//!
//! ## Data access layer
//!
//! You can interact with data access layer through the [`DbCtx`](db::DbCtx) object,
//! which contains information about the target database and the specific query builder
//! used for that database implementation.
//!
//! With [`DbOps`](db::DbOps) in scope, you can call CRUD methods on [`DbCtx`](db::DbCtx) with [`Entity`] that implements:
//! - [`GetEntity`](entity::ops::GetEntity)
//! - [`ListEntity`](entity::ops::ListEntity)
//! - [`CreateEntity`](entity::ops::CreateEntity)
//! - [`UpdateEntity`](entity::ops::UpdateEntity)
//! - [`DeleteEntity`](entity::ops::DeleteEntity)
//!
//! ```
//! use chrono::{DateTime, Utc};
//! use lazybe::db::DbOps;
//! use lazybe::db::sqlite::SqliteDbCtx;
//! use lazybe::macros::Entity;
//! use sqlx::{Executor, SqlitePool};
//!
//! #[derive(Debug, PartialEq, Eq, Entity)]
//! #[lazybe(table = "todo")]
//! pub struct Todo {
//!     #[lazybe(primary_key)]
//!     pub id: i32,
//!     pub title: String,
//!     pub description: Option<String>,
//!     pub is_completed: bool,
//!     #[lazybe(created_at)]
//!     pub created_at: DateTime<Utc>,
//!     #[lazybe(updated_at)]
//!     pub updated_at: DateTime<Utc>,
//! }
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let pool = SqlitePool::connect("sqlite::memory:").await?;
//!     pool.execute(
//!         r#"
//!          CREATE TABLE IF NOT EXISTS todo (
//!              id INTEGER PRIMARY KEY AUTOINCREMENT,
//!              title TEXT NOT NULL,
//!              description TEXT,
//!              is_completed BOOLEAN NOT NULL,
//!              created_at DATETIME NOT NULL,
//!              updated_at DATETIME NOT NULL
//!          );
//!          "#,
//!     )
//!     .await?;
//!
//!     // This is the main object we use to interact with the database
//!     let ctx = SqliteDbCtx;
//!
//!     // Create a new todo record
//!     let mut tx = pool.begin().await?;
//!     let create_todo = CreateTodo {
//!         title: "My first todo".to_string(),
//!         description: None,
//!         is_completed: false,
//!     };
//!     let todo_1 = ctx.create::<Todo>(&mut tx, create_todo).await?;
//!     tx.commit().await?;
//!
//!     // Read a record from the database
//!     let mut tx = pool.begin().await?;
//!     let todo_2 = ctx.get::<Todo>(&mut tx, todo_1.id).await?.unwrap();
//!     tx.commit().await?;
//!
//!     assert_eq!(todo_1, todo_2);
//!     Ok(())
//! }
//! ```
//!
//!
//! ## API layer
//!
//! An [`Entity`] can be exposed on an [`axum`] router with `endpoint` attribute.
//! You also need to implement the [`RouteConfig`](router::RouteConfig)
//! on the shared state so the router impls can obtain the required context to perform CRUD operations.
//!
//! By providing the `endpoint` attribute, these traits are automatically implemented.
//! - [`Routable`](router::Routable)
//! - [`ListRouter`](router::ListRouter)
//! - [`GetRouter`](router::GetRouter)
//! - [`CreateRouter`](router::CreateRouter)
//! - [`UpdateRouter`](router::UpdateRouter)
//! - [`DeleteRouter`](router::DeleteRouter)
//!
//! ```
//! use chrono::{DateTime, Utc};
//! use lazybe::axum::Router;
//! use lazybe::db::sqlite::SqliteDbCtx;
//! use lazybe::macros::Entity;
//! use lazybe::router::{
//!     CreateRouter, DeleteRouter, GetRouter, ListRouter, RouteConfig, UpdateRouter,
//! };
//! use serde::{Deserialize, Serialize};
//! use sqlx::{Executor, Sqlite, SqlitePool};
//!
//! #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Entity)]
//! #[lazybe(table = "todo", endpoint = "/todos")]
//! pub struct Todo {
//!     #[lazybe(primary_key)]
//!     pub id: i32,
//!     pub title: String,
//!     pub description: Option<String>,
//!     pub is_completed: bool,
//!     #[lazybe(created_at)]
//!     pub created_at: DateTime<Utc>,
//!     #[lazybe(updated_at)]
//!     pub updated_at: DateTime<Utc>,
//! }
//!
//! #[derive(Clone)]
//! struct AppState {
//!     ctx: SqliteDbCtx,
//!     pool: SqlitePool,
//! }
//!
//! // Make sure the DAL has access to what it needs from the shared axum state
//! impl RouteConfig for AppState {
//!     type Ctx = SqliteDbCtx;
//!     type Db = Sqlite;
//!
//!     fn db_ctx(&self) -> (Self::Ctx, SqlitePool) {
//!         (self.ctx.clone(), self.pool.clone())
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let ctx = SqliteDbCtx;
//!     let pool = SqlitePool::connect("sqlite::memory:").await?;
//!
//!     // Use this router to compose your axum application
//!     let router: Router = Router::new()
//!         .merge(Todo::get_endpoint())
//!         .merge(Todo::create_endpoint())
//!         .merge(Todo::update_endpoint())
//!         .merge(Todo::replace_endpoint())
//!         .merge(Todo::delete_endpoint())
//!         .with_state(AppState { ctx, pool });
//!
//!     Ok(())
//! }
//! ```
//!
//! See also:
//! - [`EntityCollectionApi`](router::EntityCollectionApi)
//! - [`ValidationHook`](router::ValidationHook)
//!
//!
//! ## OpenAPI documentation
//!
//! This crate utilizes [`utoipa`] to generate an OpenAPI documentation.
//! For your own types, you can derive [`ToSchema`](utoipa::ToSchema) directly to generate OpenAPI documentation.
//! For sibling types generated by the [`Entity`](macros::Entity) macro, you can use the `derive_to_schema` attribute
//! to derive [`ToSchema`](utoipa::ToSchema) for all of them.
//!
//! [`CreateRouterDoc`](openapi::CreateRouterDoc), [`UpdateRouterDoc`](openapi::UpdateRouterDoc), etc
//! should be automatically implemented for an [`Entity`] if [`ToSchema`](utoipa::ToSchema) is implemented.
//!
//! ```
//! use chrono::{DateTime, Utc};
//! use lazybe::macros::Entity;
//! use lazybe::openapi::{
//!     CreateRouterDoc, DeleteRouterDoc, GetRouterDoc, ListRouterDoc, UpdateRouterDoc,
//! };
//! use serde::{Deserialize, Serialize};
//! use utoipa::ToSchema;
//! use utoipa::openapi::{Info, OpenApi, OpenApiBuilder, Server};
//!
//! #[derive(Serialize, Deserialize, ToSchema, Entity)]
//! #[lazybe(table = "todo", endpoint = "/todos", derive_to_schema)]
//! pub struct Todo {
//!     #[lazybe(primary_key)]
//!     pub id: i32,
//!     pub title: String,
//!     pub description: Option<String>,
//!     pub is_completed: bool,
//!     #[lazybe(created_at)]
//!     pub created_at: DateTime<Utc>,
//!     #[lazybe(updated_at)]
//!     pub updated_at: DateTime<Utc>,
//! }
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Use this OpenApi to compose your utoipa documentation
//!     let openapi: OpenApi = OpenApiBuilder::new()
//!         .info(Info::new("Todo Example", "0.1.0"))
//!         .servers(Some([Server::new("http://localhost:8080")]))
//!         .build()
//!         .merge_from(Todo::get_endpoint_doc(None))
//!         .merge_from(Todo::list_endpoint_doc(None))
//!         .merge_from(Todo::create_endpoint_doc(None))
//!         .merge_from(Todo::update_endpoint_doc(None))
//!         .merge_from(Todo::replace_endpoint_doc(None))
//!         .merge_from(Todo::delete_endpoint_doc(None));
//!
//!     Ok(())
//! }
//! ```
//!
//! See also:
//! - [`openapi`] module
//!
//!
//! # Advanced Usage
//!
//!
//! ## Validation
//! See [`ValidationHook`](router::ValidationHook)
//!
//!
//! ## Custom collection API
//! See [`EntityCollectionApi`](router::EntityCollectionApi)
//!
//!
//! ## Custom ID generation
//!
//! The field that serves as the primary key is usually generated from the database.
//! If you need control over how the ID is generated, you can use the `generate_with` attribute to
//! specify the function used for ID generation. This function accepts the reference to [`Entity::Create`] type.
//!
//! ```
//! use lazybe::macros::Entity;
//! use lazybe::uuid::Uuid;
//!
//! #[derive(Entity)]
//! #[lazybe(table = "todo")]
//! pub struct Todo {
//!     #[lazybe(primary_key, generate_with = "uuid_string")]
//!     pub id: String,
//!     pub title: String,
//!     pub description: Option<String>,
//!     pub is_completed: bool,
//! }
//!
//! fn uuid_string(_: &CreateTodo) -> String {
//!     Uuid::new_v4().to_string()
//! }
//! ```
//!
//!
//! ## Nested types
//!
//! The [`Entity`](macros::Entity) macro can generate building blocks from the API layer all the way
//! to the database layer. Since it is mapped directly to a database table,
//! its structure is largely constrained by the table-like format.
//!
//! Depending on how you map the complex types to the table format, the following technique can be used.
//! - Use JSON column type
//! - Implement entity operation manually
//! - Use only DAL and write [`axum`] route manually
//!
//! ### Use JSON column type
//!
//! This encodes the `Author` as a JSON column in the `book` table.
//!
//! ```
//! use lazybe::macros::Entity;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, Entity)]
//! #[lazybe(table = "book", endpoint = "/books")]
//! pub struct Book {
//!     #[lazybe(primary_key)]
//!     pub id: i32,
//!     pub title: String,
//!     #[lazybe(json)]
//!     pub author: Author,
//! }
//!
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! pub struct Author {
//!     pub first_name: String,
//!     pub last_name: String,
//!     pub pen_name: Option<String>,
//! };
//! ```
//!
//! ### Implement entity operation manually
//!
//! Here, we are creating a facade which can have complex structure.
//! Then we use the DAL to implement a [`CreateEntity`](entity::ops::CreateEntity)
//! which automatically implements a [`CreateRouter`](router::CreateRouter).
//!
//! ```
//! use lazybe::db::{DbCtx, DbOps};
//! use lazybe::entity::ops::CreateEntity;
//! use lazybe::macros::{Entity, EntityEndpoint};
//! use serde::{Deserialize, Serialize};
//! use sqlx::Sqlite;
//!
//! #[derive(Serialize, Deserialize, Entity)]
//! #[lazybe(table = "book")]
//! pub struct Book {
//!     #[lazybe(primary_key)]
//!     pub id: i32,
//!     pub title: String,
//!     pub author_id: i32,
//! }
//!
//! #[derive(Serialize, Deserialize, Entity)]
//! #[lazybe(table = "author")]
//! pub struct Author {
//!     #[lazybe(primary_key)]
//!     pub id: i32,
//!     pub first_name: String,
//!     pub last_name: String,
//!     pub pen_name: Option<String>,
//! };
//!
//! #[derive(Serialize, Deserialize, EntityEndpoint)]
//! #[lazybe(endpoint = "/books", create_ty = "CreateBookFacade")]
//! pub struct BookFacade {
//!     #[serde(flatten)]
//!     pub book: Book,
//!     pub author: Author,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! pub struct CreateBookFacade {
//!     pub book: String,
//!     pub author: CreateAuthor,
//! }
//!
//! impl CreateEntity<Sqlite> for BookFacade {
//!     async fn create<Ctx>(
//!         ctx: &Ctx,
//!         tx: &mut sqlx::Transaction<'_, Sqlite>,
//!         input: Self::Create,
//!     ) -> Result<Self, sqlx::Error>
//!     where
//!         Ctx: DbCtx<Sqlite> + Sync,
//!     {
//!         let author = ctx.create::<Author>(tx, input.author).await?;
//!         let create_book = CreateBook {
//!             title: input.book,
//!             author_id: author.id,
//!         };
//!         let book = ctx.create::<Book>(tx, create_book).await?;
//!         Ok(Self { book, author })
//!     }
//! }
//!
//! // You can now call BookFacade::create_endpoint()
//! ```
//!
//! See also:
//! - [`EntityEndpoint`](macros::EntityEndpoint)

pub use entity::{Entity, TableEntity};

/// Database interactions
pub mod db;
/// Traits and types for describing entity
pub mod entity;
/// Utilities for filtering records
pub mod filter;
/// Utilities for pagination
pub mod page;
/// Triats and types for querying entities on a database
pub mod query;
/// Utilities for sorting records
pub mod sort;

/// Re-exports of [`uuid`]
pub mod uuid {
    pub use uuid::*;
}

/// Re-exports of proc-macro
pub mod macros {
    pub use lazybe_macro::*;
}

/// Module implementing [`axum`] router
#[cfg(feature = "axum")]
pub mod router;

/// Re-exports of [`axum`]
#[cfg(feature = "axum")]
pub mod axum {
    pub use axum::*;
}

/// Utilities for generating a OpenAPI documentation
#[cfg(feature = "openapi")]
pub mod openapi;
