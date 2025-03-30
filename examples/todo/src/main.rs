use lazybe::DbCtx;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{Executor, Pool, Sqlite, SqlitePool};

#[derive(Debug, Clone, PartialEq, Eq, lazybe::DalNewtype)]
pub struct StaffId(u64);

#[derive(Debug, Clone, PartialEq, Eq, lazybe::DalEntity)]
#[lazybe(table = "staff")]
pub struct Staff {
    #[lazybe(primary_key)]
    pub id: StaffId,
    pub name: String,
    #[lazybe(created_at)]
    pub created_at: DateTime<Utc>,
    #[lazybe(updated_at)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, lazybe::DalNewtype)]
pub struct TodoId(u64);

#[derive(Debug, Clone, PartialEq, Eq, lazybe::DalEntity)]
#[lazybe(table = "todo")]
pub struct Todo {
    #[lazybe(primary_key)]
    pub id: TodoId,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub assignee: Option<StaffId>,
    #[lazybe(created_at)]
    pub created_at: DateTime<Utc>,
    #[lazybe(updated_at)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, lazybe::DalEnum)]
pub enum Status {
    Backlog,
    Todo,
    Doing,
    Done,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    let ctx = DbCtx::sqlite();
    run_migration(&pool).await?;

    let alice: Staff = ctx
        .create(
            &pool,
            CreateStaff {
                name: "alice".to_string(),
            },
        )
        .await?;

    let new_todo = CreateTodo {
        title: "Optimize slow database query".to_string(),
        description: None,
        status: Status::Backlog,
        assignee: Some(alice.id),
    };

    let todo1: Todo = ctx.create(&pool, new_todo).await?;
    println!("Todo added to db with id {:?}", todo1.id);

    let todo2: Todo = ctx.get(&pool, todo1.id.clone()).await?.unwrap();
    println!("Todo read from db: {:#?}", todo2);

    assert_eq!(todo1, todo2);
    Ok(())
}

async fn run_migration(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    pool.execute(
        r#"
CREATE TABLE IF NOT EXISTS staff (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    assignee INTEGER REFERENCES staff(id) ON DELETE RESTRICT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);
        "#,
    )
    .await?;
    Ok(())
}

