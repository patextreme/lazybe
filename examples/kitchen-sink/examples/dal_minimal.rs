use chrono::{DateTime, NaiveDate, Utc};
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
    };
    let create_book_2 = CreateBook {
        title: "Harry Potter 2".to_string(),
        author: "J. K. Rowling".to_string(),
        publication_date: NaiveDate::from_ymd_opt(1998, 7, 2).unwrap(),
    };

    let mut tx = pool.begin().await?;
    let book_1 = ctx.create::<Book>(&mut tx, create_book_1).await?;
    let book_2 = ctx.create::<Book>(&mut tx, create_book_2).await?;
    tx.commit().await?;

    println!("book_1: {:?}", book_1);
    println!("book_2: {:?}", book_2);

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
    #[lazybe(created_at)]
    pub created_at: DateTime<Utc>,
}

async fn migrate(pool: &SqlitePool) -> anyhow::Result<()> {
    pool.execute(
        r#"
CREATE TABLE IF NOT EXISTS book (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    publication_date DATE NOT NULL,
    created_at DATETIME NOT NULL
);
        "#,
    )
    .await?;
    Ok(())
}
