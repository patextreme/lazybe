use projection::CountResult;
use sea_query::{Alias, Asterisk, Expr, QueryBuilder};
use sqlx::{Database, Executor, FromRow, IntoArguments};

use crate::filter::Filter;
use crate::sort::Sort;
use crate::{CreateQuery, DeleteQuery, GetQuery, ListQuery, Page, PaginationInput, UpdateQuery};

pub trait DbCtx<Db> {
    type Qb: QueryBuilder + Default + Send;
    fn query_buidler(&self) -> Self::Qb;
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    use sea_query::SqliteQueryBuilder;
    use sqlx::Sqlite;

    use super::DbCtx;

    #[derive(Debug, Clone, Default)]
    pub struct SqliteDbCtx;

    impl DbCtx<Sqlite> for SqliteDbCtx {
        type Qb = SqliteQueryBuilder;

        fn query_buidler(&self) -> Self::Qb {
            SqliteQueryBuilder
        }
    }
}

#[cfg(feature = "postgres")]
pub mod postgres {
    use sea_query::PostgresQueryBuilder;
    use sqlx::Postgres;

    use super::DbCtx;

    #[derive(Debug, Clone, Default)]
    pub struct PostgresDbCtx;

    impl DbCtx<Postgres> for PostgresDbCtx {
        type Qb = PostgresQueryBuilder;

        fn query_buidler(&self) -> Self::Qb {
            PostgresQueryBuilder
        }
    }
}

pub trait DbOps<Db>
where
    Db: Database,
{
    fn get<'e, T, E>(
        &self,
        executor: E,
        id: <T as GetQuery>::Pk,
    ) -> impl Future<Output = Result<Option<T>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = Db>,
        T: GetQuery,
        <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin;

    fn list<'e, T, E>(
        &self,
        executor: E,
        filter: Filter<T>,
        sort: Sort<T>,
        pagination: Option<PaginationInput>,
    ) -> impl Future<Output = Result<Page<T>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = Db> + Clone,
        T: ListQuery,
        <T as ListQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
        CountResult: for<'r> FromRow<'r, Db::Row> + Send + Unpin;

    fn create<'e, T, E>(
        &self,
        executor: E,
        input: <T as CreateQuery>::Create,
    ) -> impl Future<Output = Result<T, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = Db>,
        T: CreateQuery,
        <T as CreateQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin;

    fn update<'e, T, E>(
        &self,
        executor: E,
        id: <T as UpdateQuery>::Pk,
        input: <T as UpdateQuery>::Update,
    ) -> impl Future<Output = Result<Option<T>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = Db>,
        T: UpdateQuery,
        <T as UpdateQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin;

    fn delete<'e, T, E>(
        &self,
        executor: E,
        id: <T as DeleteQuery>::Pk,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send
    where
        T: DeleteQuery,
        E: Executor<'e, Database = Db>;
}

impl<Ctx, Db> DbOps<Db> for Ctx
where
    Ctx: DbCtx<Db>,
    Db: Database,
    for<'q> <Db as Database>::Arguments<'q>: IntoArguments<'q, Db>,
{
    fn get<'e, T, E>(
        &self,
        executor: E,
        id: <T as GetQuery>::Pk,
    ) -> impl Future<Output = Result<Option<T>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = Db>,
        T: GetQuery,
        <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
    {
        let query = <T as GetQuery>::get_query(id).to_string(self.query_buidler());
        async move {
            let maybe_entity: Option<<T as GetQuery>::Row> = sqlx::query_as(&query).fetch_optional(executor).await?;
            Ok(maybe_entity.map(|i| i.into()))
        }
    }

    fn list<'e, T, E>(
        &self,
        executor: E,
        filter: Filter<T>,
        sort: Sort<T>,
        pagination: Option<PaginationInput>,
    ) -> impl Future<Output = Result<Page<T>, sqlx::Error>> + Send
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
            stm.to_owned().to_string(self.query_buidler())
        };

        // data
        let data_query = {
            // sort
            let order_by = sort.into_order_exprs();
            base_query = base_query.order_by_columns(order_by).to_owned();

            // filter
            if let Some(p) = &pagination {
                base_query = base_query.limit(p.limit.into()).offset(p.offset().into()).to_owned();
            }
            base_query.to_string(self.query_buidler())
        };

        async move {
            let data_result: Vec<<T as ListQuery>::Row> =
                sqlx::query_as(&data_query).fetch_all(executor.clone()).await?;
            let count_result: CountResult = sqlx::query_as(&count_query).fetch_one(executor).await?;

            let mut page: u32 = 0;
            let mut page_size: u64 = count_result.count.unsigned_abs();
            if let Some(p) = pagination {
                page = p.page;
                page_size = p.limit.into();
            }

            let result = Page {
                page,
                page_size,
                total_records: count_result.count.unsigned_abs(),
                data: data_result.into_iter().map(|i| i.into()).collect(),
            };
            Ok(result)
        }
    }

    fn create<'e, T, E>(
        &self,
        executor: E,
        input: <T as CreateQuery>::Create,
    ) -> impl Future<Output = Result<T, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = Db>,
        T: CreateQuery,
        <T as CreateQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
    {
        let query = <T as CreateQuery>::create_query(input).to_string(self.query_buidler());
        async move {
            let entity: <T as CreateQuery>::Row = sqlx::query_as(&query).fetch_one(executor).await?;
            Ok(entity.into())
        }
    }

    fn update<'e, T, E>(
        &self,
        executor: E,
        id: <T as UpdateQuery>::Pk,
        input: <T as UpdateQuery>::Update,
    ) -> impl Future<Output = Result<Option<T>, sqlx::Error>> + Send
    where
        E: Executor<'e, Database = Db>,
        T: UpdateQuery,
        <T as UpdateQuery>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
    {
        let query = <T as UpdateQuery>::update_query(id, input).to_string(self.query_buidler());
        async move {
            let maybe_entity: Option<<T as UpdateQuery>::Row> = sqlx::query_as(&query).fetch_optional(executor).await?;
            Ok(maybe_entity.map(|i| i.into()))
        }
    }

    fn delete<'e, T, E>(
        &self,
        executor: E,
        id: <T as DeleteQuery>::Pk,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send
    where
        T: DeleteQuery,
        E: Executor<'e, Database = Db>,
    {
        let query = <T as DeleteQuery>::delete_query(id).to_string(self.query_buidler());
        async move {
            sqlx::query(&query).execute(executor).await?;
            Ok(())
        }
    }
}

mod projection {
    #[derive(sqlx::FromRow)]
    pub struct CountResult {
        pub count: i64,
    }
}
