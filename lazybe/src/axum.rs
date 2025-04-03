use axum::Router;
use sqlx::{Database, Pool};

use crate::DbOps;

pub trait Routable {
    fn entity_path() -> &'static str;
    fn entity_collection_path() -> &'static str;
}

pub trait GetRouter<S, Db> {
    fn get_router() -> Router<S>;
}

pub trait CreateRouter<S, Db> {
    fn create_router() -> Router<S>;
}

pub trait DeleteRouter<S, Db> {
    fn delete_router() -> Router<S>;
}

pub trait ToDbState {
    type Ctx: DbOps<Self::Db>;
    type Db: Database;

    fn to_db_state(&self) -> (Self::Ctx, Pool<Self::Db>);
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::routing::{delete, get, post};
    use axum::{Json, Router};
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use sqlx::{Database, FromRow};

    use super::{CreateRouter, DeleteRouter, GetRouter, Routable, ToDbState};
    use crate::{CreateQuery, DbOps, DeleteQuery, GetQuery};

    super::macros::axum_route_impl!(sqlx::Sqlite, crate::db::sqlite::SqliteDbCtx);
}

#[cfg(feature = "postgres")]
pub mod postgres {
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::routing::{delete, get, post};
    use axum::{Json, Router};
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use sqlx::{Database, FromRow};

    use super::{CreateRouter, DeleteRouter, GetRouter, Routable, ToDbState};
    use crate::{CreateQuery, DbOps, DeleteQuery, GetQuery};

    super::macros::axum_route_impl!(sqlx::Postgres, crate::db::postgres::PostgresDbCtx);
}

mod macros {
    macro_rules! axum_route_impl {
        ($db_ty:ty, $ctx_ty:ty) => {
            type DbImpl = $db_ty;
            type CtxImpl = $ctx_ty;

            impl<T, S> GetRouter<S, DbImpl> for T
            where
                T: GetQuery + Routable + Serialize + 'static,
                S: ToDbState<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as GetQuery>::Pk: DeserializeOwned + Send,
                <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                fn get_router() -> Router<S> {
                    let route = <T as Routable>::entity_path();
                    Router::new().route(route, get(get_router_impl::<T, S>))
                }
            }

            async fn get_router_impl<T, S>(
                Path(id): Path<<T as GetQuery>::Pk>,
                State(state): State<S>,
            ) -> Result<Json<T>, StatusCode>
            where
                T: GetQuery + Routable + Serialize + 'static,
                S: ToDbState<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as GetQuery>::Pk: DeserializeOwned + Send,
                <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                let (ctx, pool) = state.to_db_state();
                let result = ctx
                    .get::<T, _>(&pool, id)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                    .ok_or(StatusCode::NOT_FOUND)?;
                Ok(Json(result))
            }

            impl<T, S> CreateRouter<S, DbImpl> for T
            where
                T: CreateQuery + Routable + Serialize + DeserializeOwned + 'static,
                S: ToDbState<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as CreateQuery>::Create: DeserializeOwned + Send,
                <T as CreateQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                fn create_router() -> Router<S> {
                    let route = <T as Routable>::entity_collection_path();
                    Router::new().route(route, post(create_router_impl::<T, S>))
                }
            }

            async fn create_router_impl<T, S>(
                State(state): State<S>,
                Json(input): Json<<T as CreateQuery>::Create>,
            ) -> Result<Json<T>, StatusCode>
            where
                T: CreateQuery + Routable + Serialize + DeserializeOwned + 'static,
                S: ToDbState<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as CreateQuery>::Create: DeserializeOwned + Send,
                <T as CreateQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                let (ctx, pool) = state.to_db_state();
                let result = ctx
                    .create::<T, _>(&pool, input)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                Ok(Json(result))
            }

            impl<T, S> DeleteRouter<S, DbImpl> for T
            where
                T: DeleteQuery + Routable + Serialize + 'static,
                S: ToDbState<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as DeleteQuery>::Pk: DeserializeOwned + Send,
            {
                fn delete_router() -> Router<S> {
                    let route = <T as Routable>::entity_path();
                    Router::new().route(route, delete(delete_router_impl::<T, S>))
                }
            }

            async fn delete_router_impl<T, S>(
                Path(id): Path<<T as DeleteQuery>::Pk>,
                State(state): State<S>,
            ) -> Result<Json<()>, StatusCode>
            where
                T: DeleteQuery + Routable + Serialize + 'static,
                S: ToDbState<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as DeleteQuery>::Pk: DeserializeOwned + Send,
            {
                let (ctx, pool) = state.to_db_state();
                ctx.delete::<T, _>(&pool, id)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                Ok(Json(()))
            }
        };
    }

    pub(super) use axum_route_impl;
}
