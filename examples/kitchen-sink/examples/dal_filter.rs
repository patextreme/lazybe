use chrono::NaiveDate;
use lazybe::db::DbOps;
use lazybe::db::sqlite::SqliteDbCtx;
use lazybe::filter::Filter;
use lazybe::macros::Entity;
use lazybe::sort::Sort;
use sqlx::{Executor, SqlitePool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ctx = SqliteDbCtx;
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    migrate(&pool).await?;
    seed(&ctx, &pool).await?;

    let mut tx = pool.begin().await?;

    let books = ctx
        .list::<Book>(&mut tx, Filter::empty(), Sort::empty(), None)
        .await?
        .data
        .into_iter()
        .map(|i| format!("{} ({})", i.title, i.publication_date))
        .collect::<Vec<_>>();

    let books_by_alphabetical_order = ctx
        .list::<Book>(&mut tx, Filter::empty(), Sort::new([BookSort::title().asc()]), None)
        .await?
        .data
        .into_iter()
        .map(|i| format!("{} ({})", i.title, i.publication_date))
        .collect::<Vec<_>>();

    let books_title_containing_of = ctx
        .list::<Book>(
            &mut tx,
            Filter::all([BookFilter::title().like("%of%")]),
            Sort::empty(),
            None,
        )
        .await?
        .data
        .into_iter()
        .map(|i| format!("{} ({})", i.title, i.publication_date))
        .collect::<Vec<_>>();

    let books_title_containing_of_or_between_2000_2010 = ctx
        .list::<Book>(
            &mut tx,
            Filter::any([
                Filter::all([
                    BookFilter::publication_date().gte(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
                    BookFilter::publication_date().lt(NaiveDate::from_ymd_opt(2011, 1, 1).unwrap()),
                ]),
                Filter::all([BookFilter::title().like("%of%")]),
            ]),
            Sort::new([BookSort::publication_date().asc()]),
            None,
        )
        .await?
        .data
        .into_iter()
        .map(|i| format!("{} ({})", i.title, i.publication_date))
        .collect::<Vec<_>>();

    println!("All books: {:#?}", books);
    println!("All books in alphabetical order: {:#?}", books_by_alphabetical_order);
    println!(
        "Books with title containing the word \"of\": {:#?}",
        books_title_containing_of
    );
    println!(
        "Books with title containing the word \"of\" or published between 2000-2010: {:#?}",
        books_title_containing_of_or_between_2000_2010
    );

    tx.commit().await?;

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
}

async fn migrate(pool: &SqlitePool) -> anyhow::Result<()> {
    pool.execute(
        r#"
CREATE TABLE IF NOT EXISTS book (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    publication_date DATE NOT NULL
);
        "#,
    )
    .await?;
    Ok(())
}

async fn seed(ctx: &SqliteDbCtx, pool: &SqlitePool) -> anyhow::Result<()> {
    let book_defs = [
        (
            "Harry Potter and the Philosopher's Stone",
            NaiveDate::from_ymd_opt(1997, 6, 26).unwrap(),
        ),
        (
            "Harry Potter and the Chamber of Secrets",
            NaiveDate::from_ymd_opt(1998, 7, 2).unwrap(),
        ),
        (
            "Harry Potter and the Prisoner of Azkaban",
            NaiveDate::from_ymd_opt(1999, 7, 8).unwrap(),
        ),
        (
            "Harry Potter and the Goblet of Fire",
            NaiveDate::from_ymd_opt(2000, 7, 8).unwrap(),
        ),
        (
            "Harry Potter and the Order of the Phoenix",
            NaiveDate::from_ymd_opt(2003, 6, 21).unwrap(),
        ),
        (
            "Harry Potter and the Half-Blood Prince",
            NaiveDate::from_ymd_opt(2005, 7, 16).unwrap(),
        ),
        (
            "Harry Potter and the Deathly Hallows",
            NaiveDate::from_ymd_opt(2007, 7, 21).unwrap(),
        ),
        (
            "Harry Potter and the Cursed Child",
            NaiveDate::from_ymd_opt(2016, 7, 30).unwrap(),
        ),
    ];

    let mut tx = pool.begin().await?;
    for (title, publication_date) in book_defs {
        ctx.create::<Book>(
            &mut tx,
            CreateBook {
                title: title.to_string(),
                author: "J. K. Rowling".to_string(),
                publication_date,
            },
        )
        .await?;
    }
    tx.commit().await?;
    Ok(())
}
