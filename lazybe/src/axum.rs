use axum::Router;
use sqlx::{Database, Pool};

use crate::DbOps;

pub trait Routable {
    fn get_route() -> &'static str;
}

pub trait GetRouter<S, Db>: Sized {
    fn get_router() -> Router<S>;
}

pub trait ToDbState {
    type Ctx: DbOps<Self::Db>;
    type Db: Database;

    fn to_db_state(&self) -> (Self::Ctx, Pool<Self::Db>);
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    super::macros::axum_route_impl!(sqlx::Sqlite, crate::db::sqlite::SqliteDbCtx);
}

#[cfg(feature = "postgres")]
pub mod postgres {
    super::macros::axum_route_impl!(sqlx::Postgres, crate::db::postgres::PostgresDbCtx);
}

mod macros {
    macro_rules! axum_route_impl {
        ($db_ty:ty, $ctx_ty:ty) => {
            use axum::extract::{Path, State};
            use axum::http::StatusCode;
            use axum::routing::get;
            use axum::{Json, Router};
            use serde::Serialize;
            use serde::de::DeserializeOwned;
            use sqlx::{Database, FromRow};

            use super::{GetRouter, Routable, ToDbState};
            use crate::{DbOps, GetQuery};

            type DbImpl = $db_ty;
            type CtxImpl = $ctx_ty;

            impl<T, S> GetRouter<S, DbImpl> for T
            where
                T: GetQuery + Routable + Serialize + DeserializeOwned + 'static,
                S: ToDbState<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as GetQuery>::Pk: DeserializeOwned + Send,
                <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                fn get_router() -> Router<S> {
                    let route = <T as Routable>::get_route();
                    Router::new().route(route, get(get_router_impl::<T, S>))
                }
            }

            async fn get_router_impl<T, S>(
                Path(id): Path<<T as GetQuery>::Pk>,
                State(state): State<S>,
            ) -> Result<Json<T>, StatusCode>
            where
                T: GetQuery + Routable + Serialize + DeserializeOwned + 'static,
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
        };
    }

    pub(super) use axum_route_impl;
}

