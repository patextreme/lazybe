use lazybe::DbCtx;
use sqlx::Executor;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod todo;

#[tokio::main]
async fn main() {
    init_tracing();

    let pool = sqlx::SqlitePool::connect("./test.db").await.unwrap();
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
    .await
    .unwrap();

    let ctx = DbCtx::sqlite();

    let new_todo = todo::CreateTodo {
        title: "todo-1".to_string(),
        description: Some("some description".to_string()),
        status: todo::Status::Backlog,
        priority: Some(todo::Priority::Medium),
    };

    let todo1 = ctx.create::<todo::Todo, _>(&pool, new_todo).await.unwrap();
    tracing::info!("created todo id {}", todo1.id);

    let todo2 = ctx.get::<todo::Todo, _>(&pool, todo1.id).await.unwrap();
    tracing::info!("received todo id {:#?}", todo2);
}

fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
