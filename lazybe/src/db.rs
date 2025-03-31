use std::marker::PhantomData;

use sqlx::{Database, Executor, FromRow, IntoArguments};

use crate::filter::Filter;
use crate::{CreateQuery, DeleteQuery, GetQuery, ListQuery, UpdateQuery};

pub struct DbCtx<Qb, Db> {
    query_builder: PhantomData<Qb>,
    db: PhantomData<Db>,
}

#[cfg(feature = "sqlite")]
impl DbCtx<sea_query::SqliteQueryBuilder, sqlx::Sqlite> {
    pub fn sqlite() -> Self {
        DbCtx {
            query_builder: PhantomData,
            db: PhantomData,
        }
    }
}

#[cfg(feature = "postgres")]
impl DbCtx<sea_query::PostgresQueryBuilder, sqlx::Postgres> {
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

    pub async fn list<'e, T, E>(&self, executor: E, filter: Filter<T>) -> Result<Vec<T>, sqlx::Error>
    where
        E: Executor<'e, Database = Db>,
        T: ListQuery,
        <T as ListQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
    {
        let query = <T as ListQuery>::list_query(filter).to_string(Qb::default());
        let entities: Vec<<T as ListQuery>::Row> = sqlx::query_as(&query).fetch_all(executor).await?;
        Ok(entities.into_iter().map(|i| i.into()).collect())
    }

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

    pub async fn update<'e, T, E>(
        &self,
        executor: E,
        id: <T as UpdateQuery>::Pk,
        input: <T as UpdateQuery>::Update,
    ) -> Result<Option<T>, sqlx::Error>
    where
        E: Executor<'e, Database = Db>,
        T: UpdateQuery,
        <T as UpdateQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
    {
        let query = <T as UpdateQuery>::update_query(id, input).to_string(Qb::default());
        let maybe_entity: Option<<T as UpdateQuery>::Row> = sqlx::query_as(&query).fetch_optional(executor).await?;
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
