use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use sea_query::QueryBuilder;
use serde::Serialize;
use serde::de::DeserializeOwned;
use sqlx::{Database, Pool};

use crate::{DbCtx, GetQuery};

pub trait RouterState {
    type Qb: QueryBuilder;
    type Db: Database;

    fn db_pool(&self) -> Pool<Self::Db>;
    fn db_ctx(&self) -> DbCtx<Self::Qb, Self::Db>;
}

pub trait GetRoutable {
    fn get_route() -> &'static str;
}

pub trait GetRouter<S>: Sized {
    type Qb: QueryBuilder;
    type Db: Database;

    fn get_router() -> Router<S>;
}

impl<T, S, Qb, Db> GetRouter<S> for T
where
    T: GetQuery + GetRoutable + Serialize + DeserializeOwned + 'static,
    S: RouterState<Qb = Qb, Db = Db> + Clone + Send + Sync + 'static,
    Db: Database,
    Qb: QueryBuilder + Default + 'static,
    <T as GetQuery>::Pk: TryFrom<String>,
{
    type Qb = Qb;
    type Db = Db;

    fn get_router() -> Router<S> {
        let route = <T as GetRoutable>::get_route();
        Router::new().route(route, get(get_router_impl::<T, S, Qb, Db>))
    }
}

// TODO: return error payload
async fn get_router_impl<T, S, Qb, Db>(Path(id): Path<String>, State(state): State<S>) -> Result<Json<T>, StatusCode>
where
    T: GetQuery + GetRoutable + Serialize + DeserializeOwned + 'static,
    S: RouterState<Qb = Qb, Db = Db> + Clone + Send + Sync + 'static,
    Db: Database,
    Qb: QueryBuilder + Default + 'static,
    <T as GetQuery>::Pk: TryFrom<String>,
{
    let parsed_id: <T as GetQuery>::Pk = <T as GetQuery>::Pk::try_from(id)
        .map_err(|e| StatusCode::BAD_REQUEST)?;

    todo!()
}

