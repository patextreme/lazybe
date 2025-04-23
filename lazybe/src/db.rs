use sea_query::QueryBuilder;
use sqlx::{Database, Transaction};

use crate::Entity;
use crate::entity::ops::{CreateEntity, DeleteEntity, GetEntity, ListEntity, UpdateEntity};
use crate::filter::Filter;
use crate::page::{Page, PaginationInput};
use crate::sort::Sort;

/// A context containing information about the target database.
pub trait DbCtx<Db> {
    type Qb: QueryBuilder + Default + Send;
    fn query_builder(&self) -> Self::Qb;
}

/// Integraion with sqlite database
#[cfg(feature = "sqlite")]
pub mod sqlite {
    use sea_query::SqliteQueryBuilder;
    use sqlx::Sqlite;

    use super::DbCtx;

    #[derive(Debug, Clone, Default)]
    pub struct SqliteDbCtx;

    impl DbCtx<Sqlite> for SqliteDbCtx {
        type Qb = SqliteQueryBuilder;

        fn query_builder(&self) -> Self::Qb {
            SqliteQueryBuilder
        }
    }
}

/// Integraion with postgres database
#[cfg(feature = "postgres")]
pub mod postgres {
    use sea_query::PostgresQueryBuilder;
    use sqlx::Postgres;

    use super::DbCtx;

    #[derive(Debug, Clone, Default)]
    pub struct PostgresDbCtx;

    impl DbCtx<Postgres> for PostgresDbCtx {
        type Qb = PostgresQueryBuilder;

        fn query_builder(&self) -> Self::Qb {
            PostgresQueryBuilder
        }
    }
}

/// Database operations
pub trait DbOps<Db>
where
    Db: Database,
{
    fn get<T>(
        &self,
        tx: &mut Transaction<'_, Db>,
        id: <T as Entity>::Pk,
    ) -> impl Future<Output = Result<Option<T>, sqlx::Error>> + Send
    where
        T: GetEntity<Db>;

    fn list<T>(
        &self,
        tx: &mut Transaction<'_, Db>,
        filter: Filter<T>,
        sort: Sort<T>,
        pagination: Option<PaginationInput>,
    ) -> impl Future<Output = Result<Page<T>, sqlx::Error>> + Send
    where
        T: ListEntity<Db>;

    fn create<T>(
        &self,
        tx: &mut Transaction<'_, Db>,
        input: <T as Entity>::Create,
    ) -> impl Future<Output = Result<T, sqlx::Error>> + Send
    where
        T: CreateEntity<Db>;

    fn update<T>(
        &self,
        tx: &mut Transaction<'_, Db>,
        id: <T as Entity>::Pk,
        input: <T as Entity>::Update,
    ) -> impl Future<Output = Result<Option<T>, sqlx::Error>> + Send
    where
        T: UpdateEntity<Db>;

    fn delete<T>(
        &self,
        tx: &mut Transaction<'_, Db>,
        id: <T as Entity>::Pk,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send
    where
        T: DeleteEntity<Db>;
}

impl<Ctx, Db> DbOps<Db> for Ctx
where
    Ctx: DbCtx<Db> + Sync,
    Db: Database,
{
    fn get<T>(
        &self,
        tx: &mut Transaction<'_, Db>,
        id: <T as Entity>::Pk,
    ) -> impl Future<Output = Result<Option<T>, sqlx::Error>> + Send
    where
        T: GetEntity<Db>,
    {
        <T as GetEntity<Db>>::get(self, tx, id)
    }

    fn list<T>(
        &self,
        tx: &mut Transaction<'_, Db>,
        filter: Filter<T>,
        sort: Sort<T>,
        pagination: Option<PaginationInput>,
    ) -> impl Future<Output = Result<Page<T>, sqlx::Error>> + Send
    where
        T: ListEntity<Db>,
    {
        <T as ListEntity<Db>>::list(self, tx, filter, sort, pagination)
    }

    fn create<T>(
        &self,
        tx: &mut Transaction<'_, Db>,
        input: <T as Entity>::Create,
    ) -> impl Future<Output = Result<T, sqlx::Error>> + Send
    where
        T: CreateEntity<Db>,
    {
        <T as CreateEntity<Db>>::create(self, tx, input)
    }

    fn update<T>(
        &self,
        tx: &mut Transaction<'_, Db>,
        id: <T as Entity>::Pk,
        input: <T as Entity>::Update,
    ) -> impl Future<Output = Result<Option<T>, sqlx::Error>> + Send
    where
        T: UpdateEntity<Db>,
    {
        <T as UpdateEntity<Db>>::update(self, tx, id, input)
    }

    fn delete<T>(
        &self,
        tx: &mut Transaction<'_, Db>,
        id: <T as Entity>::Pk,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send
    where
        T: DeleteEntity<Db>,
    {
        <T as DeleteEntity<Db>>::delete(self, tx, id)
    }
}
