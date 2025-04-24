use std::ops::DerefMut;

use projection::CountResult;
use sea_query::{Alias, Asterisk, Expr};
use sqlx::{Database, Executor, FromRow, IntoArguments, Transaction};

use crate::db::DbCtx;
use crate::filter::Filter;
use crate::page::{Page, PaginationInput};
use crate::query::{CreateQuery, DeleteQuery, GetQuery, ListQuery, UpdateQuery};
use crate::sort::Sort;
use crate::{Entity, TableEntity};

pub trait GetEntity<Db>: Entity
where
    Db: Database,
{
    fn get<Ctx>(
        ctx: &Ctx,
        tx: &mut Transaction<'_, Db>,
        id: Self::Pk,
    ) -> impl Future<Output = Result<Option<Self>, sqlx::Error>> + Send
    where
        Ctx: DbCtx<Db> + Sync;
}

pub trait ListEntity<Db>: Entity
where
    Db: Database,
{
    fn list<Ctx>(
        ctx: &Ctx,
        tx: &mut Transaction<'_, Db>,
        filter: Filter<Self>,
        sort: Sort<Self>,
        pagination: Option<PaginationInput>,
    ) -> impl Future<Output = Result<Page<Self>, sqlx::Error>> + Send
    where
        Ctx: DbCtx<Db> + Sync;
}

pub trait CreateEntity<Db>: Entity
where
    Db: Database,
{
    fn create<Ctx>(
        ctx: &Ctx,
        tx: &mut Transaction<'_, Db>,
        input: Self::Create,
    ) -> impl Future<Output = Result<Self, sqlx::Error>> + Send
    where
        Ctx: DbCtx<Db> + Sync;
}

pub trait UpdateEntity<Db>: Entity
where
    Db: Database,
{
    fn update<Ctx>(
        ctx: &Ctx,
        tx: &mut Transaction<'_, Db>,
        id: Self::Pk,
        input: Self::Update,
    ) -> impl Future<Output = Result<Option<Self>, sqlx::Error>> + Send
    where
        Ctx: DbCtx<Db> + Sync;
}

pub trait DeleteEntity<Db>: Entity
where
    Db: Database,
{
    fn delete<Ctx>(
        ctx: &Ctx,
        tx: &mut Transaction<'_, Db>,
        id: Self::Pk,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send
    where
        Ctx: DbCtx<Db> + Sync;
}

impl<T, Db> GetEntity<Db> for T
where
    Db: Database,
    T: GetQuery,
    for<'q> <Db as Database>::Arguments<'q>: IntoArguments<'q, Db>,
    for<'c> &'c mut <Db as Database>::Connection: Executor<'c, Database = Db>,
    <T as TableEntity>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
{
    fn get<Ctx>(
        ctx: &Ctx,
        tx: &mut Transaction<'_, Db>,
        id: Self::Pk,
    ) -> impl Future<Output = Result<Option<Self>, sqlx::Error>> + Send
    where
        Ctx: DbCtx<Db> + Sync,
    {
        let query = <T as GetQuery>::get_query(id).to_string(ctx.query_builder());
        async move {
            let maybe_entity: Option<<T as TableEntity>::Row> =
                sqlx::query_as(&query).fetch_optional(tx.deref_mut()).await?;
            Ok(maybe_entity.map(|i| i.into()))
        }
    }
}

impl<T, Db> ListEntity<Db> for T
where
    Db: Database,
    T: ListQuery,
    for<'q> <Db as Database>::Arguments<'q>: IntoArguments<'q, Db>,
    for<'c> &'c mut <Db as Database>::Connection: Executor<'c, Database = Db>,
    <T as TableEntity>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
    CountResult: for<'r> FromRow<'r, Db::Row> + Send + Unpin,
{
    fn list<Ctx>(
        ctx: &Ctx,
        tx: &mut Transaction<'_, Db>,
        filter: Filter<Self>,
        sort: Sort<Self>,
        pagination: Option<PaginationInput>,
    ) -> impl Future<Output = Result<Page<Self>, sqlx::Error>> + Send
    where
        Ctx: DbCtx<Db> + Sync,
    {
        let mut base_query = <T as ListQuery>::list_query(filter);

        // count
        let count_query = {
            let mut stm = base_query.clone();
            stm.clear_selects()
                .clear_order_by()
                .expr_as(Expr::col(Asterisk).count(), Alias::new("count"));
            stm.to_owned().to_string(ctx.query_builder())
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
            base_query.to_string(ctx.query_builder())
        };

        async move {
            let data_result: Vec<<T as TableEntity>::Row> =
                sqlx::query_as(&data_query).fetch_all(tx.deref_mut()).await?;

            let count_result: CountResult = sqlx::query_as(&count_query).fetch_one(tx.deref_mut()).await?;

            let total_records: u32 = count_result
                .count
                .try_into()
                .expect("record count does not fit in u32");
            let mut page: u32 = 0;
            let mut page_size: u32 = total_records;
            if let Some(p) = pagination {
                page = p.page;
                page_size = p.limit.into();
            }

            let result = Page {
                page,
                page_size,
                total_records,
                data: data_result.into_iter().map(|i| i.into()).collect(),
            };
            Ok(result)
        }
    }
}

impl<T, Db> CreateEntity<Db> for T
where
    Db: Database,
    T: CreateQuery,
    for<'q> <Db as Database>::Arguments<'q>: IntoArguments<'q, Db>,
    for<'c> &'c mut <Db as Database>::Connection: Executor<'c, Database = Db>,
    <T as TableEntity>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
{
    fn create<Ctx>(
        ctx: &Ctx,
        tx: &mut sqlx::Transaction<'_, Db>,
        input: Self::Create,
    ) -> impl Future<Output = Result<Self, sqlx::Error>> + Send
    where
        Ctx: DbCtx<Db> + Sync,
    {
        let query = <T as CreateQuery>::create_query(input).to_string(ctx.query_builder());
        async move {
            let entity: <T as TableEntity>::Row = sqlx::query_as(&query).fetch_one(tx.deref_mut()).await?;
            Ok(entity.into())
        }
    }
}

impl<T, Db> UpdateEntity<Db> for T
where
    Db: Database,
    T: UpdateQuery,
    for<'q> <Db as Database>::Arguments<'q>: IntoArguments<'q, Db>,
    for<'c> &'c mut <Db as Database>::Connection: Executor<'c, Database = Db>,
    <T as TableEntity>::Row: Into<T> + for<'r> FromRow<'r, Db::Row> + Send + Unpin,
{
    fn update<Ctx>(
        ctx: &Ctx,
        tx: &mut Transaction<'_, Db>,
        id: Self::Pk,
        input: Self::Update,
    ) -> impl Future<Output = Result<Option<Self>, sqlx::Error>> + Send
    where
        Ctx: DbCtx<Db> + Sync,
    {
        let query = <T as UpdateQuery>::update_query(id, input).to_string(ctx.query_builder());
        async move {
            let maybe_entity: Option<<T as TableEntity>::Row> =
                sqlx::query_as(&query).fetch_optional(tx.deref_mut()).await?;
            Ok(maybe_entity.map(|i| i.into()))
        }
    }
}

impl<T, Db> DeleteEntity<Db> for T
where
    Db: Database,
    T: DeleteQuery,
    for<'q> <Db as Database>::Arguments<'q>: IntoArguments<'q, Db>,
    for<'c> &'c mut <Db as Database>::Connection: Executor<'c, Database = Db>,
{
    fn delete<Ctx>(
        ctx: &Ctx,
        tx: &mut Transaction<'_, Db>,
        id: Self::Pk,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send
    where
        Ctx: DbCtx<Db> + Sync,
    {
        let query = <T as DeleteQuery>::delete_query(id).to_string(ctx.query_builder());
        async move {
            sqlx::query(&query).execute(tx.deref_mut()).await?;
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
