pub use axum::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sqlx::{Database, Pool};
use uuid::Uuid;

use crate::filter::Filter;
use crate::sort::Sort;
use crate::{DbOps, Page, PaginationInput};

/// https://www.rfc-editor.org/rfc/rfc9457#name-type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "oas", derive(utoipa::ToSchema))]
pub struct ErrorResponse {
    title: String,
    detail: Option<String>,
    instance: Option<String>,
}

impl ErrorResponse {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            detail: None,
            instance: None,
        }
    }

    pub fn with_detail(self, detail: &str) -> Self {
        Self {
            detail: Some(detail.to_string()),
            ..self
        }
    }

    pub fn with_instance(self, id: Uuid) -> Self {
        Self {
            instance: Some(id.to_string()),
            ..self
        }
    }
}

pub trait EntityCollectionApi: Sized {
    type Resp: Serialize;
    type Query: DeserializeOwned;

    fn page_response(page: Page<Self>) -> Self::Resp;

    fn page_input(input: &Self::Query) -> Option<PaginationInput>;
    fn filter_input(input: &Self::Query) -> Filter<Self>;
    fn sort_input(input: &Self::Query) -> Sort<Self>;
}

pub trait Routable {
    fn entity_path() -> &'static str;
    fn entity_collection_path() -> &'static str;
}

pub trait GetRouter<S, Db> {
    fn get_endpoint() -> Router<S>;
}

pub trait ListRouter<S, Db> {
    fn list_endpoint() -> Router<S>;
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

pub trait RouteConfig {
    type Ctx: DbOps<Self::Db>;
    type Db: Database;

    fn db_ctx(&self) -> (Self::Ctx, Pool<Self::Db>);
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    use axum::extract::{Path, Query, State};
    use axum::http::StatusCode;
    use axum::routing::{delete, get, patch, post, put};
    use axum::{Json, Router};
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use sqlx::{Database, FromRow};
    use uuid::Uuid;

    use super::{
        CreateRouter, DeleteRouter, EntityCollectionApi, ErrorResponse, GetRouter, ListRouter, Routable, RouteConfig,
        UpdateRouter,
    };
    use crate::{CreateQuery, DbOps, DeleteQuery, GetQuery, ListQuery, UpdateQuery};

    super::macros::axum_route_impl!(sqlx::Sqlite, crate::db::sqlite::SqliteDbCtx);
}

#[cfg(feature = "postgres")]
pub mod postgres {
    use axum::extract::{Path, Query, State};
    use axum::http::StatusCode;
    use axum::routing::{delete, get, patch, post, put};
    use axum::{Json, Router};
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use sqlx::{Database, FromRow};
    use uuid::Uuid;

    use super::{
        CreateRouter, DeleteRouter, EntityCollectionApi, ErrorResponse, GetRouter, ListRouter, Routable, RouteConfig,
        UpdateRouter,
    };
    use crate::{CreateQuery, DbOps, DeleteQuery, GetQuery, ListQuery, UpdateQuery};

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
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
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
            ) -> Result<Json<T>, (StatusCode, Json<ErrorResponse>)>
            where
                T: GetQuery + Routable + Serialize + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as GetQuery>::Pk: DeserializeOwned + Send,
                <T as GetQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                let (ctx, pool) = state.db_ctx();
                let url = <T as Routable>::entity_path();
                let result = ctx
                    .get::<T, _>(&pool, id.clone())
                    .await
                    .map_err(|e| {
                        let instance = Uuid::new_v4();
                        tracing::error!(?instance, ?url, ?id, "Failed to get an entity: {}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                ErrorResponse::new("Internal server error")
                                    .with_detail("Unknown error occurred, please check the logs for more details.")
                                    .with_instance(instance),
                            ),
                        )
                    })?
                    .ok_or((
                        StatusCode::NOT_FOUND,
                        Json(
                            ErrorResponse::new("Not found")
                                .with_detail(&format!("An entity with id {:?} was not found.", id)),
                        ),
                    ))?;
                Ok(Json(result))
            }

            impl<T, S> ListRouter<S, DbImpl> for T
            where
                T: ListQuery + EntityCollectionApi + Routable + Serialize + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as ListQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
                <T as EntityCollectionApi>::Query: Send,
            {
                fn list_endpoint() -> Router<S> {
                    let route = <T as Routable>::entity_collection_path();
                    Router::new().route(route, get(list_endpoint_impl::<T, S>))
                }
            }

            async fn list_endpoint_impl<T, S>(
                State(state): State<S>,
                Query(query): Query<<T as EntityCollectionApi>::Query>,
            ) -> Result<Json<<T as EntityCollectionApi>::Resp>, (StatusCode, Json<ErrorResponse>)>
            where
                T: ListQuery + EntityCollectionApi + Routable + Serialize + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as ListQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
                <T as EntityCollectionApi>::Query: Send,
            {
                let (ctx, pool) = state.db_ctx();
                let url = <T as Routable>::entity_collection_path();

                let page_input = <T as EntityCollectionApi>::page_input(&query);
                let filter_input = <T as EntityCollectionApi>::filter_input(&query);
                let sort_input = <T as EntityCollectionApi>::sort_input(&query);
                let result = ctx
                    .list::<T, _>(&pool, filter_input, sort_input, page_input)
                    .await
                    .map_err(|e| {
                        let instance = Uuid::new_v4();
                        tracing::error!(?instance, ?url, "Failed to list entities: {}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                ErrorResponse::new("Internal server error")
                                    .with_detail("Unknown error occurred, please check the logs for more details.")
                                    .with_instance(instance),
                            ),
                        )
                    })?;
                let page_resp = <T as EntityCollectionApi>::page_response(result);
                Ok(Json(page_resp))
            }

            impl<T, S> CreateRouter<S, DbImpl> for T
            where
                T: CreateQuery + Routable + Serialize + DeserializeOwned + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
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
            ) -> Result<(StatusCode, Json<T>), (StatusCode, Json<ErrorResponse>)>
            where
                T: CreateQuery + Routable + Serialize + DeserializeOwned + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as CreateQuery>::Create: DeserializeOwned + Send,
                <T as CreateQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                let (ctx, pool) = state.db_ctx();
                let url = <T as Routable>::entity_collection_path();
                let result = ctx.create::<T, _>(&pool, input).await.map_err(|e| {
                    let instance = Uuid::new_v4();
                    tracing::error!(?instance, ?url, "Failed to create an entity: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            ErrorResponse::new("Internal server error")
                                .with_detail("Unknown error occurred, please check the logs for more details.")
                                .with_instance(instance),
                        ),
                    )
                })?;
                Ok((StatusCode::CREATED, Json(result)))
            }

            impl<T, S> DeleteRouter<S, DbImpl> for T
            where
                T: DeleteQuery + Routable + Serialize + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
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
            ) -> Result<Json<()>, (StatusCode, Json<ErrorResponse>)>
            where
                T: DeleteQuery + Routable + Serialize + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as DeleteQuery>::Pk: DeserializeOwned + Send,
            {
                let (ctx, pool) = state.db_ctx();
                let url = <T as Routable>::entity_path();
                ctx.delete::<T, _>(&pool, id.clone()).await.map_err(|e| {
                    let instance = Uuid::new_v4();
                    tracing::error!(?instance, ?url, ?id, "Failed to delete an entity: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            ErrorResponse::new("Internal server error")
                                .with_detail("Unknown error occurred, please check the logs for more details.")
                                .with_instance(instance),
                        ),
                    )
                })?;
                Ok(Json(()))
            }

            impl<T, S> UpdateRouter<S, DbImpl> for T
            where
                T: UpdateQuery + Routable + Serialize + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
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
            ) -> Result<Json<T>, (StatusCode, Json<ErrorResponse>)>
            where
                T: UpdateQuery + Routable + Serialize + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as UpdateQuery>::Pk: DeserializeOwned + Send,
                <T as UpdateQuery>::Update: DeserializeOwned + Send,
                <T as UpdateQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                let (ctx, pool) = state.db_ctx();
                let url = <T as Routable>::entity_path();
                let result = ctx
                    .update::<T, _>(&pool, id.clone(), input)
                    .await
                    .map_err(|e| {
                        let instance = Uuid::new_v4();
                        tracing::error!(?instance, ?url, ?id, "Failed to update an entity: {}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                ErrorResponse::new("Internal server error")
                                    .with_detail("Unknown error occurred, please check the logs for more details.")
                                    .with_instance(instance),
                            ),
                        )
                    })?
                    .ok_or((
                        StatusCode::NOT_FOUND,
                        Json(
                            ErrorResponse::new("Not found")
                                .with_detail(&format!("An entity with id {:?} was not found.", id)),
                        ),
                    ))?;
                Ok(Json(result))
            }

            async fn replace_endpoint_impl<T, S>(
                Path(id): Path<<T as UpdateQuery>::Pk>,
                State(state): State<S>,
                Json(input): Json<<T as UpdateQuery>::Replace>,
            ) -> Result<Json<T>, (StatusCode, Json<ErrorResponse>)>
            where
                T: UpdateQuery + Routable + Serialize + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as UpdateQuery>::Pk: DeserializeOwned + Send,
                <T as UpdateQuery>::Replace: DeserializeOwned + Send,
                <T as UpdateQuery>::Row: Into<T> + for<'r> FromRow<'r, <DbImpl as Database>::Row> + Send + Unpin,
            {
                let (ctx, pool) = state.db_ctx();
                let url = <T as Routable>::entity_path();
                let patch_input: <T as UpdateQuery>::Update = input.into();
                let result = ctx
                    .update::<T, _>(&pool, id.clone(), patch_input)
                    .await
                    .map_err(|e| {
                        let instance = Uuid::new_v4();
                        tracing::error!(?instance, ?url, ?id, "Failed to replace an entity: {}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                ErrorResponse::new("Internal server error")
                                    .with_detail("Unknown error occurred, please check the logs for more details.")
                                    .with_instance(instance),
                            ),
                        )
                    })?
                    .ok_or((
                        StatusCode::NOT_FOUND,
                        Json(
                            ErrorResponse::new("Not found")
                                .with_detail(&format!("An entity with id {:?} was not found.", id)),
                        ),
                    ))?;
                Ok(Json(result))
            }
        };
    }

    pub(super) use axum_route_impl;
}
