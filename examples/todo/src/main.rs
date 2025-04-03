use lazybe::filter::Filter;
use lazybe::sort::Sort;
use lazybe::{DbCtx, SqliteDbCtx};
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
    let ctx: SqliteDbCtx = DbCtx::sqlite();
    run_migration(&pool).await?;

    let alice: Staff = ctx
        .create(
            &pool,
            CreateStaff {
                name: "Alice".to_string(),
            },
        )
        .await?;

    let todo_defs = [("Do homework", Status::Doing), ("Wash car", Status::Todo)];
    for (title, status) in todo_defs {
        ctx.create::<Todo, _>(
            &pool,
            CreateTodo {
                title: title.to_string(),
                description: None,
                status,
                assignee: Some(alice.id.clone()),
            },
        )
        .await?;
    }

    let tasks = ctx.list_all::<Todo, _>(&pool, Filter::empty(), Sort::empty()).await?;
    println!(">>>>> All tasks <<<<<\n{:#?}", tasks);

    // Alice pick up a task and complete it
    let alice_incomplete_tasks = ctx
        .list_all::<Todo, _>(
            &pool,
            Filter::all([
                TodoFilter::assignee().eq(Some(alice.id.clone())),
                TodoFilter::status().neq(Status::Done),
            ]),
            Sort::empty(),
        )
        .await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    ctx.update::<Todo, _>(
        &pool,
        alice_incomplete_tasks.first().cloned().unwrap().id,
        UpdateTodo {
            status: Some(Status::Done),
            ..Default::default()
        },
    )
    .await?;

    let completed_tasks = ctx
        .list_all(
            &pool,
            Filter::all([TodoFilter::status().eq(Status::Done)]),
            Sort::empty(),
        )
        .await?;
    println!(">>>>> Completed tasks <<<<<\n{:#?}", completed_tasks);

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
