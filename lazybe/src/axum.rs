use axum::Router;

pub trait GetRoutable {
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

    use super::{GetRoutable, GetRouter};
    use crate::GetQuery;

    pub trait ToSqliteAxumState {
        fn to_sqlite_state(&self) -> (crate::db::SqliteDbCtx, sqlx::Pool<sqlx::Sqlite>);
    }

    impl<T, S> GetRouter<S> for T
    where
        T: GetQuery + GetRoutable + Serialize + DeserializeOwned + 'static,
        S: ToSqliteAxumState + Clone + Send + Sync + 'static,
        <T as GetQuery>::Pk: DeserializeOwned + Send,
        <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
    {
        fn get_router() -> Router<S> {
            let route = <T as GetRoutable>::get_route();
            Router::new().route(route, get(get_router_impl::<T, S>))
        }
    }

    async fn get_router_impl<T, S>(Path(id): Path<<T as GetQuery>::Pk>, State(state): State<S>) -> Result<Json<T>, StatusCode>
    where
        T: GetQuery + GetRoutable + Serialize + DeserializeOwned + 'static,
        S: ToSqliteAxumState + Clone + Send + Sync + 'static,
        <T as GetQuery>::Pk: DeserializeOwned + Send,
        <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
    {
        let (ctx, pool) = state.to_sqlite_state();
        let result = ctx
            .get::<T, _>(&pool, id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;
        Ok(Json(result))
    }
}
