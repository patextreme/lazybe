use chrono::NaiveDate;
use lazybe::db::DbOps;
use lazybe::db::sqlite::SqliteDbCtx;
use lazybe::macros::Entity;
use lazybe::uuid;
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

    let mut tx = pool.begin().await?;
    let book_1 = ctx.create::<Book>(&mut tx, create_book_1).await?;
    tx.commit().await?;

    println!("book_1: {:?}", book_1);

    Ok(())
}

#[derive(Debug, Clone, Entity)]
#[lazybe(table = "book")]
pub struct Book {
    #[lazybe(primary_key, generate_with = "random_uuid")]
    pub id: uuid::fmt::Hyphenated,
    pub title: String,
    pub author: String,
    pub publication_date: NaiveDate,
}

fn random_uuid(_: &CreateBook) -> uuid::fmt::Hyphenated {
    uuid::Uuid::new_v4().into()
}

async fn migrate(pool: &SqlitePool) -> anyhow::Result<()> {
    pool.execute(
        r#"
CREATE TABLE IF NOT EXISTS book (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    publication_date DATE NOT NULL
);
        "#,
    )
    .await?;
    Ok(())
}
