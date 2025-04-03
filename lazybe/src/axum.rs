use axum::Router;
use sqlx::{Database, Pool};

use crate::DbOps;

pub trait Routable {
    fn entity_path() -> &'static str;
    fn entity_collection_path() -> &'static str;
}

pub trait GetRouter<S, Db> {
    fn get_endpoint() -> Router<S>;
}

pub trait CreateRouter<S, Db> {
    fn create_endpoint() -> Router<S>;
}

pub trait UpdateRouter<S, Db> {
    fn update_endpoint() -> Router<S>;
    fn replace_endpoint() -> Router<S>;
}

pub trait DeleteRouter<S, Db> {
    fn delete_endpoint() -> Router<S>;
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
    use axum::routing::{delete, get, patch, post, put};
    use axum::{Json, Router};
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use sqlx::{Database, FromRow};

    use super::{CreateRouter, DeleteRouter, GetRouter, Routable, ToDbState, UpdateRouter};
    use crate::{CreateQuery, DbOps, DeleteQuery, GetQuery, UpdateQuery};

    super::macros::axum_route_impl!(sqlx::Sqlite, crate::db::sqlite::SqliteDbCtx);
}

#[cfg(feature = "postgres")]
pub mod postgres {
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::routing::{delete, get, patch, post, put};
    use axum::{Json, Router};
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use sqlx::{Database, FromRow};

    use super::{CreateRouter, DeleteRouter, GetRouter, Routable, ToDbState, UpdateRouter};
    use crate::{CreateQuery, DbOps, DeleteQuery, GetQuery, UpdateQuery};

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
                fn get_endpoint() -> Router<S> {
                    let route = <T as Routable>::entity_path();
                    Router::new().route(route, get(get_endpoint_impl::<T, S>))
                }
            }

            async fn get_endpoint_impl<T, S>(
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
                fn create_endpoint() -> Router<S> {
                    let route = <T as Routable>::entity_collection_path();
                    Router::new().route(route, post(create_endpoint_impl::<T, S>))
                }
            }

            async fn create_endpoint_impl<T, S>(
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
                fn delete_endpoint() -> Router<S> {
                    let route = <T as Routable>::entity_path();
                    Router::new().route(route, delete(delete_endpoint_impl::<T, S>))
                }
            }

            async fn delete_endpoint_impl<T, S>(
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

            impl<T, S> UpdateRouter<S, DbImpl> for T
            where
                T: UpdateQuery + Routable + Serialize + 'static,
                S: ToDbState<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as UpdateQuery>::Pk: DeserializeOwned + Send,
                <T as UpdateQuery>::Update: DeserializeOwned + Send,
                <T as UpdateQuery>::Replace: DeserializeOwned + Send,
                <T as UpdateQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                fn replace_endpoint() -> Router<S> {
                    let route = <T as Routable>::entity_path();
                    Router::new().route(route, put(replace_endpoint_impl::<T, S>))
                }

                fn update_endpoint() -> Router<S> {
                    let route = <T as Routable>::entity_path();
                    Router::new().route(route, patch(update_endpoint_impl::<T, S>))
                }
            }

            async fn update_endpoint_impl<T, S>(
                Path(id): Path<<T as UpdateQuery>::Pk>,
                State(state): State<S>,
                Json(input): Json<<T as UpdateQuery>::Update>,
            ) -> Result<Json<T>, StatusCode>
            where
                T: UpdateQuery + Routable + Serialize + 'static,
                S: ToDbState<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as UpdateQuery>::Pk: DeserializeOwned + Send,
                <T as UpdateQuery>::Update: DeserializeOwned + Send,
                <T as UpdateQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                let (ctx, pool) = state.to_db_state();
                let result = ctx
                    .update::<T, _>(&pool, id, input)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                    .ok_or(StatusCode::NOT_FOUND)?;
                Ok(Json(result))
            }

            async fn replace_endpoint_impl<T, S>(
                Path(id): Path<<T as UpdateQuery>::Pk>,
                State(state): State<S>,
                Json(input): Json<<T as UpdateQuery>::Replace>,
            ) -> Result<Json<T>, StatusCode>
            where
                T: UpdateQuery + Routable + Serialize + 'static,
                S: ToDbState<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as UpdateQuery>::Pk: DeserializeOwned + Send,
                <T as UpdateQuery>::Replace: DeserializeOwned + Send,
                <T as UpdateQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                let (ctx, pool) = state.to_db_state();
                let patch_input: <T as UpdateQuery>::Update = input.into();
                let result = ctx
                    .update::<T, _>(&pool, id, patch_input)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                    .ok_or(StatusCode::NOT_FOUND)?;
                Ok(Json(result))
            }
        };
    }

    pub(super) use axum_route_impl;
}
