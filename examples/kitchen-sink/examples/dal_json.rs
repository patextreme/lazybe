use chrono::NaiveDate;
use lazybe::db::DbOps;
use lazybe::db::sqlite::SqliteDbCtx;
use lazybe::macros::Entity;
use sqlx::{Executor, SqlitePool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ctx = SqliteDbCtx;
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    migrate(&pool).await?;

    let create_book_1 = CreateBook {
        title: "Harry Potter 1".to_string(),
        author: "J. K. Rowling".to_string(),
        publication_date: NaiveDate::from_ymd_opt(1997, 6, 26).unwrap(),
        tags: vec!["fiction".to_string(), "children".to_string()],
    };

    let mut tx = pool.begin().await?;
    let book_1 = ctx.create::<Book>(&mut tx, create_book_1).await?;
    tx.commit().await?;

    println!("book_1: {:?}", book_1);
    assert_eq!(book_1.tags, vec!["fiction".to_string(), "children".to_string(),]);

    Ok(())
}

#[derive(Debug, Clone, Entity)]
#[lazybe(table = "book")]
pub struct Book {
    #[lazybe(primary_key)]
    pub id: u32,
    pub title: String,
    pub author: String,
    pub publication_date: NaiveDate,
    #[lazybe(json)]
    pub tags: Vec<String>,
}

async fn migrate(pool: &SqlitePool) -> anyhow::Result<()> {
    pool.execute(
        r#"
CREATE TABLE IF NOT EXISTS book (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    publication_date DATE NOT NULL,
    tags TEXT NOT NULL
);
        "#,
    )
    .await?;
    Ok(())
}
