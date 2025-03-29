use std::marker::PhantomData;

use sea_query::{PostgresQueryBuilder, SqliteQueryBuilder};
use sqlx::{Database, Executor, FromRow, IntoArguments, Postgres, Sqlite};

use crate::{CreateQuery, DeleteQuery, GetQuery};

pub struct DbCtx<Qb, Db> {
    query_builder: PhantomData<Qb>,
    db: PhantomData<Db>,
}

impl DbCtx<SqliteQueryBuilder, Sqlite> {
    pub fn sqlite() -> Self {
        DbCtx {
            query_builder: PhantomData,
            db: PhantomData,
        }
    }
}

impl DbCtx<PostgresQueryBuilder, Postgres> {
    pub fn postgres() -> Self {
        DbCtx {
            query_builder: PhantomData,
            db: PhantomData,
        }
    }
}

impl<Qb, Db> DbCtx<Qb, Db>
where
    Qb: sea_query::QueryBuilder + Default,
    Db: Database,
    for<'q> <Db as Database>::Arguments<'q>: IntoArguments<'q, Db>,
{
    pub async fn create<'e, T, E>(&self, executor: E, input: <T as CreateQuery>::Create) -> Result<T, sqlx::Error>
    where
        E: Executor<'e, Database = Db>,
        T: CreateQuery,
        <T as CreateQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
    {
        let query = <T as CreateQuery>::create_query(input).to_string(Qb::default());
        let entity: <T as CreateQuery>::Row = sqlx::query_as(&query).fetch_one(executor).await?;
        Ok(entity.into())
    }

    pub async fn get<'e, T, E>(&self, executor: E, id: <T as GetQuery>::Pk) -> Result<Option<T>, sqlx::Error>
    where
        E: Executor<'e, Database = Db>,
        T: GetQuery,
        <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
    {
        let query = <T as GetQuery>::get_query(id).to_string(Qb::default());
        let maybe_entity: Option<<T as GetQuery>::Row> = sqlx::query_as(&query).fetch_optional(executor).await?;
        Ok(maybe_entity.map(|i| i.into()))
    }

    pub async fn delete<'e, T, E>(&self, executor: E, id: <T as DeleteQuery>::Pk) -> Result<(), sqlx::Error>
    where
        T: DeleteQuery,
        E: Executor<'e, Database = Db>,
    {
        let query = <T as DeleteQuery>::delete_query(id).to_string(Qb::default());
        sqlx::query(&query).execute(executor).await?;
        Ok(())
    }
}
