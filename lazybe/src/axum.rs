use axum::Router;

pub trait Routable {
    fn get_route() -> &'static str;
}

pub trait GetRouter<S>: Sized {
    fn get_router() -> Router<S>;
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::routing::get;
    use axum::{Json, Router};
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use sqlx::FromRow;

    use super::{Routable, GetRouter};
    use crate::GetQuery;

    type DbCtx = crate::db::sqlite::SqliteDbCtx;
    type DbPool = sqlx::Pool<sqlx::Sqlite>;
    type DbRow = sqlx::sqlite::SqliteRow;

    pub trait ToDbState {
        fn to_db_state(&self) -> (DbCtx, DbPool);
    }

    impl<T, S> GetRouter<S> for T
    where
        T: GetQuery + Routable + Serialize + DeserializeOwned + 'static,
        S: ToDbState + Clone + Send + Sync + 'static,
        <T as GetQuery>::Pk: DeserializeOwned + Send,
        <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, DbRow> + Send + Unpin,
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
        S: ToDbState + Clone + Send + Sync + 'static,
        <T as GetQuery>::Pk: DeserializeOwned + Send,
        <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, DbRow> + Send + Unpin,
    {
        let (ctx, pool) = state.to_db_state();
        let result = ctx
            .get::<T, _>(&pool, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;
        Ok(Json(result))
    }
}

#[cfg(feature = "postgres")]
pub mod postgres {
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::routing::get;
    use axum::{Json, Router};
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use sqlx::FromRow;

    use super::{GetRoutable, GetRouter};
    use crate::GetQuery;

    type DbCtx = crate::db::PostgresDbCtx;
    type DbPool = sqlx::Pool<sqlx::Postgres>;
    type DbRow = sqlx::postgres::PgRow;

    pub trait ToDbState {
        fn to_db_state(&self) -> (DbCtx, DbPool);
    }

    impl<T, S> GetRouter<S> for T
    where
        T: GetQuery + Routable + Serialize + DeserializeOwned + 'static,
        S: ToDbState + Clone + Send + Sync + 'static,
        <T as GetQuery>::Pk: DeserializeOwned + Send,
        <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, DbRow> + Send + Unpin,
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
        S: ToDbState + Clone + Send + Sync + 'static,
        <T as GetQuery>::Pk: DeserializeOwned + Send,
        <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, DbRow> + Send + Unpin,
    {
        let (ctx, pool) = state.to_db_state();
        let result = ctx
            .get::<T, _>(&pool, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;
        Ok(Json(result))
    }
}
