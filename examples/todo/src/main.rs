use lazybe::DbCtx;
use sqlx::{Executor, Pool, Sqlite, SqlitePool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    run_migration(&pool).await?;

    let ctx = DbCtx::sqlite();

    let new_todo = CreateTodo {
        title: "Get some milk from supermarket".to_string(),
        description: None,
        status: Status::Todo,
        priority: Some(Priority::Medium),
    };

    let todo1: Todo = ctx.create(&pool, new_todo).await?;
    println!("Todo added to db with id {}", todo1.id);

    let todo2: Option<Todo> = ctx.get(&pool, todo1.id).await?;
    println!("Todo read from db: {:?}", todo2);

    assert_eq!(todo1, todo2.unwrap());
    Ok(())
}

async fn run_migration(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    pool.execute(
        r#"
CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    priority TEXT
);
        "#,
    )
    .await?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, lazybe::Dal)]
#[lazybe(table = "todo")]
pub struct Todo {
    #[lazybe(primary_key)]
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub priority: Option<Priority>,
}

#[derive(Debug, Clone, PartialEq, Eq, lazybe::DalEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, PartialEq, Eq, lazybe::DalEnum)]
pub enum Status {
    Backlog,
    Todo,
    Doing,
    Done,
}
