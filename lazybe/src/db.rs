use std::marker::PhantomData;

use projection::CountResult;
use sea_query::{Alias, Asterisk, Expr};
use sqlx::{Database, Executor, FromRow, IntoArguments};

use crate::filter::Filter;
use crate::sort::Sort;
use crate::{CreateQuery, DeleteQuery, GetQuery, ListQuery, Page, Pagination, UpdateQuery};

pub struct DbCtx<Qb, Db> {
    query_builder: PhantomData<Qb>,
    db: PhantomData<Db>,
}

#[cfg(feature = "sqlite")]
pub type SqliteDbCtx = DbCtx<sea_query::SqliteQueryBuilder, sqlx::Sqlite>;

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
pub type PostgresDbCtx = DbCtx<sea_query::PostgresQueryBuilder, sqlx::Postgres>;

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

    pub async fn list_all<'e, T, E>(&self, executor: E, filter: Filter<T>, sort: Sort<T>) -> Result<Vec<T>, sqlx::Error>
    where
        E: Executor<'e, Database = Db> + Clone,
        T: ListQuery,
        <T as ListQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
        CountResult: for<'r> FromRow<'r, Db::Row> + Send + Unpin,
    {
        let page_result = self.list_page(executor, filter, sort, None).await?;
        Ok(page_result.data)
    }

    pub async fn list_page<'e, T, E>(
        &self,
        executor: E,
        filter: Filter<T>,
        sort: Sort<T>,
        pagination: Option<Pagination>,
    ) -> Result<Page<T>, sqlx::Error>
    where
        E: Executor<'e, Database = Db> + Clone,
        T: ListQuery,
        <T as ListQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
        CountResult: for<'r> FromRow<'r, Db::Row> + Send + Unpin,
    {
        let mut base_query = <T as ListQuery>::list_query(filter);

        // count
        let count_query = {
            let mut stm = base_query.clone();
            stm.clear_selects()
                .clear_order_by()
                .expr_as(Expr::col(Asterisk).count(), Alias::new("count"));
            stm.to_owned().to_string(Qb::default())
        };

        // data
        let data_query = {
            // sort
            let order_by = sort.into_order_exprs();
            base_query = base_query.order_by_columns(order_by).to_owned();

            // filter
            if let Some(p) = pagination {
                base_query = base_query.limit(p.limit).offset(p.offset()).to_owned();
            }
            base_query.to_string(Qb::default())
        };

        let data_result: Vec<<T as ListQuery>::Row> = sqlx::query_as(&data_query).fetch_all(executor.clone()).await?;
        let count_result: CountResult = sqlx::query_as(&count_query).fetch_one(executor).await?;
        let result = Page {
            data: data_result.into_iter().map(|i| i.into()).collect(),
            total: count_result.count,
        };
        Ok(result)
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

mod projection {
    #[derive(sqlx::FromRow)]
    pub struct CountResult {
        pub count: u64,
    }
}
