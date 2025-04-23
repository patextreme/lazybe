use axum::http::{Method, StatusCode};
use axum::{Json, Router};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sqlx::{Database, Pool};
use uuid::Uuid;

use crate::Entity;
use crate::db::DbOps;
use crate::filter::Filter;
use crate::page::{Page, PaginationInput};
use crate::sort::Sort;

/// A subset of properties outlined in
/// <https://www.rfc-editor.org/rfc/rfc9457#name-members-of-a-problem-detail>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ErrorResponse {
    pub title: String,
    pub detail: Option<String>,
    pub instance: Option<String>,
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

/// Defines how a collection API [`ListRouter`] behave
///
/// By default, the [`Entity`](crate::macros::Entity) macro generates a simple list collection API.
/// The list collection API returns a simple list containing all records and does not support
/// filtering, sorting or pagination.
///
/// # Example
///
/// You can also manually implement the [`EntityCollectionApi`], as different applications have
/// their own conventions, pagination methods, or may require custom syntax to support
/// sorting, filtering, and pagination logic.
///
/// ```
/// # use lazybe::filter::Filter;
/// # use lazybe::macros::Entity;
/// # use lazybe::page::{Page, PaginationInput};
/// # use lazybe::router::EntityCollectionApi;
/// # use lazybe::sort::Sort;
/// # use serde::{Deserialize, Serialize};
/// #[derive(Serialize, Entity)]
/// #[lazybe(table = "todo", endpoint = "/todos", collection_api = "manual")]
/// pub struct Todo {
///     #[lazybe(primary_key)]
///     pub id: i32,
///     pub title: String,
///     pub description: Option<String>,
///     pub is_completed: bool,
/// }
///
/// #[derive(Deserialize)]
/// pub struct TodoQuery {
///     page: Option<u32>,
///     completed: Option<bool>,
/// }
///
/// impl EntityCollectionApi for Todo {
///     type Resp = Vec<Todo>; // This could be a pagination result containing paging metadata
///     type Query = TodoQuery;
///
///     fn page_response(page: Page<Self>) -> Self::Resp {
///         page.data
///     }
///
///     fn page_input(input: &Self::Query) -> Option<PaginationInput> {
///         Some(PaginationInput {
///             // API uses 1-index, lazybe uses 0-index
///             page: input.page.map(|i| i.max(1)).unwrap_or(1) - 1,
///             limit: 100,
///         })
///     }
///
///     fn filter_input(input: &Self::Query) -> Filter<Self> {
///         // Apply filter only if provided explicitly in the query parameter
///         match input.completed {
///             Some(completed) => Filter::all([TodoFilter::is_completed().eq(completed)]),
///             None => Filter::empty(),
///         }
///     }
///
///     fn sort_input(_input: &Self::Query) -> Sort<Self> {
///         Sort::new([TodoSort::id().asc()])
///     }
/// }
/// ```
pub trait EntityCollectionApi: Sized {
    /// A collection response. Possibly a list or paginated data with some metadata.
    type Resp: Serialize;

    /// A shape of query parameter on a collection endpoint.
    /// These params are used to define pagination, filtering, and sorting.
    type Query: DeserializeOwned;

    /// Defines how a page should be translated into a collection response.
    fn page_response(page: Page<Self>) -> Self::Resp;

    /// Construct a pagination input from query parameters.
    /// Return `None` to list all records.
    fn page_input(input: &Self::Query) -> Option<PaginationInput>;

    /// Construct collection filter from query parameters
    fn filter_input(input: &Self::Query) -> Filter<Self>;

    /// Construct collection sorting from query parameters
    fn sort_input(input: &Self::Query) -> Sort<Self>;
}

/// A validation logic that gets called before and after database modifications.
///
/// When the validation returns error, the endpoint returns `400 BadRequest`
/// and the database transaction is rolled back.
///
/// # Example
///
/// ```
/// use chrono::{DateTime, Utc};
/// use lazybe::macros::Entity;
/// use lazybe::router::{ErrorResponse, ValidationHook};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Entity)]
/// #[lazybe(table = "todo", endpoint = "/todos", validation = "manual")]
/// pub struct Todo {
///     #[lazybe(primary_key)]
///     pub id: i32,
///     pub title: String,
///     pub description: Option<String>,
///     pub is_completed: bool,
///     #[lazybe(created_at)]
///     pub created_at: DateTime<Utc>,
///     #[lazybe(updated_at)]
///     pub updated_at: DateTime<Utc>,
/// }
///
/// impl ValidationHook for Todo {
///     fn before_create(input: &Self::Create) -> Result<(), ErrorResponse> {
///         if input.title.len() > 100 {
///             Err(ErrorResponse {
///                 title: "Invalid todo title".to_string(),
///                 detail: Some("Title cannot be longer than 100 characters".to_string()),
///                 instance: None,
///             })?
///         }
///         Ok(())
///     }
/// }
/// ```
pub trait ValidationHook: Entity {
    #[allow(unused_variables)]
    fn before_create(input: &Self::Create) -> Result<(), ErrorResponse> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn after_create(entity: &Self) -> Result<(), ErrorResponse> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn before_update(pk: &Self::Pk, _input: &Self::Update) -> Result<(), ErrorResponse> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn after_update(entity: &Self) -> Result<(), ErrorResponse> {
        Ok(())
    }
}

/// Describes a URL path for an [`Entity`]
pub trait Routable {
    /// A URL path for an entity with its ID (e.g. `/books/{id}`)
    fn entity_path() -> &'static str;
    /// A URL path for a collection of entity (e.g. `/books`)
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

/// Describes how to extract a configuration from a shared [`axum`] state.
pub trait RouteConfig {
    type Ctx: DbOps<Self::Db>;
    type Db: Database;

    fn db_ctx(&self) -> (Self::Ctx, Pool<Self::Db>);
}

trait ResultExt<T> {
    fn map_err_500<U: Entity>(
        self,
        method: &Method,
        url: &str,
        msg: &str,
        id: Option<&<U as Entity>::Pk>,
    ) -> Result<T, (StatusCode, Json<ErrorResponse>)>;
}

impl<T, E: std::error::Error> ResultExt<T> for Result<T, E> {
    fn map_err_500<U: Entity>(
        self,
        method: &Method,
        url: &str,
        msg: &str,
        id: Option<&<U as Entity>::Pk>,
    ) -> Result<T, (StatusCode, Json<ErrorResponse>)> {
        self.map_err(|e| {
            let instance = Uuid::new_v4();
            let entity = U::entity_name();
            tracing::error!(?instance, ?method, ?url, ?entity, ?id, "{}: {}", msg, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    ErrorResponse::new("Internal server error")
                        .with_detail("Unknown error occurred, please check the logs for more details.")
                        .with_instance(instance),
                ),
            )
        })
    }
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    super::macros::axum_route_impl_imports!();
    super::macros::axum_route_impl!(sqlx::Sqlite, crate::db::sqlite::SqliteDbCtx);
}

#[cfg(feature = "postgres")]
pub mod postgres {
    super::macros::axum_route_impl_imports!();
    super::macros::axum_route_impl!(sqlx::Postgres, crate::db::postgres::PostgresDbCtx);
}

mod macros {
    macro_rules! axum_route_impl_imports {
        () => {
            use axum::extract::{Path, Query, State};
            use axum::http::{Method, StatusCode};
            use axum::routing::{delete, get, patch, post, put};
            use axum::{Json, Router};
            use serde::Serialize;
            use serde::de::DeserializeOwned;

            use super::{
                CreateRouter, DeleteRouter, EntityCollectionApi, ErrorResponse, GetRouter, ListRouter, ResultExt,
                Routable, RouteConfig, UpdateRouter, ValidationHook,
            };
            use crate::Entity;
            use crate::db::DbOps;
            use crate::entity::ops::{CreateEntity, DeleteEntity, GetEntity, ListEntity, UpdateEntity};
        };
    }

    macro_rules! axum_route_impl {
        ($db_ty:ty, $ctx_ty:ty) => {
            type DbImpl = $db_ty;
            type CtxImpl = $ctx_ty;

            impl<T, S> GetRouter<S, DbImpl> for T
            where
                T: GetEntity<DbImpl> + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as Entity>::Pk: DeserializeOwned + Send,
            {
                fn get_endpoint() -> Router<S> {
                    let route = <T as Routable>::entity_path();
                    Router::new().route(route, get(get_endpoint_impl::<T, S>))
                }
            }

            async fn get_endpoint_impl<T, S>(
                Path(id): Path<<T as Entity>::Pk>,
                State(state): State<S>,
            ) -> Result<Json<T>, (StatusCode, Json<ErrorResponse>)>
            where
                T: GetEntity<DbImpl> + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + 'static,
                <T as Entity>::Pk: DeserializeOwned + Send,
            {
                let (ctx, pool) = state.db_ctx();
                let method = Method::GET;
                let url = <T as Routable>::entity_path();
                let mut tx = pool.begin().await.map_err_500::<T>(
                    &method,
                    url,
                    "Failed to acquire a database transaction",
                    None,
                )?;
                let result = ctx
                    .get::<T>(&mut tx, id.clone())
                    .await
                    .map_err_500::<T>(&method, url, "Failed to get an entity from database", Some(&id))?
                    .ok_or((
                        StatusCode::NOT_FOUND,
                        Json(
                            ErrorResponse::new("Not found")
                                .with_detail(&format!("An entity with id {:?} was not found.", id)),
                        ),
                    ))?;
                tx.commit()
                    .await
                    .map_err_500::<T>(&method, url, "Failed to commit a transaction", None)?;
                Ok(Json(result))
            }

            impl<T, S> ListRouter<S, DbImpl> for T
            where
                T: ListEntity<DbImpl> + EntityCollectionApi + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
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
                T: ListEntity<DbImpl> + EntityCollectionApi + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + 'static,
                <T as EntityCollectionApi>::Query: Send,
            {
                let (ctx, pool) = state.db_ctx();
                let method = Method::GET;
                let url = <T as Routable>::entity_collection_path();
                let mut tx = pool.begin().await.map_err_500::<T>(
                    &method,
                    url,
                    "Failed to acquire a database transaction",
                    None,
                )?;
                let page_input = <T as EntityCollectionApi>::page_input(&query);
                let filter_input = <T as EntityCollectionApi>::filter_input(&query);
                let sort_input = <T as EntityCollectionApi>::sort_input(&query);
                let result = ctx
                    .list::<T>(&mut tx, filter_input, sort_input, page_input)
                    .await
                    .map_err_500::<T>(&Method::GET, url, "Failed to list entities from database", None)?;
                tx.commit()
                    .await
                    .map_err_500::<T>(&method, url, "Failed to commit a transaction", None)?;
                let page_resp = <T as EntityCollectionApi>::page_response(result);
                Ok(Json(page_resp))
            }

            impl<T, S> CreateRouter<S, DbImpl> for T
            where
                T: CreateEntity<DbImpl> + ValidationHook + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as Entity>::Create: DeserializeOwned + Send,
            {
                fn create_endpoint() -> Router<S> {
                    let route = <T as Routable>::entity_collection_path();
                    Router::new().route(route, post(create_endpoint_impl::<T, S>))
                }
            }

            async fn create_endpoint_impl<T, S>(
                State(state): State<S>,
                Json(input): Json<<T as Entity>::Create>,
            ) -> Result<(StatusCode, Json<T>), (StatusCode, Json<ErrorResponse>)>
            where
                T: CreateEntity<DbImpl> + ValidationHook + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as Entity>::Create: DeserializeOwned + Send,
            {
                let (ctx, pool) = state.db_ctx();
                let method = Method::POST;
                let url = <T as Routable>::entity_collection_path();
                let mut tx = pool.begin().await.map_err_500::<T>(
                    &method,
                    url,
                    "Failed to acquire a database transaction",
                    None,
                )?;

                <T as ValidationHook>::before_create(&input).map_err(|e| (StatusCode::BAD_REQUEST, Json(e)))?;
                let result = ctx.create::<T>(&mut tx, input).await.map_err_500::<T>(
                    &method,
                    url,
                    "Failed to create an entity in database",
                    None,
                )?;
                <T as ValidationHook>::after_create(&result).map_err(|e| (StatusCode::BAD_REQUEST, Json(e)))?;

                tx.commit()
                    .await
                    .map_err_500::<T>(&method, url, "Failed to commit a transaction", None)?;
                Ok((StatusCode::CREATED, Json(result)))
            }

            impl<T, S> DeleteRouter<S, DbImpl> for T
            where
                T: DeleteEntity<DbImpl> + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as Entity>::Pk: DeserializeOwned + Send,
            {
                fn delete_endpoint() -> Router<S> {
                    let route = <T as Routable>::entity_path();
                    Router::new().route(route, delete(delete_endpoint_impl::<T, S>))
                }
            }

            async fn delete_endpoint_impl<T, S>(
                Path(id): Path<<T as Entity>::Pk>,
                State(state): State<S>,
            ) -> Result<Json<()>, (StatusCode, Json<ErrorResponse>)>
            where
                T: DeleteEntity<DbImpl> + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + 'static,
                <T as Entity>::Pk: DeserializeOwned + Send,
            {
                let (ctx, pool) = state.db_ctx();
                let method = Method::DELETE;
                let url = <T as Routable>::entity_path();
                let mut tx = pool.begin().await.map_err_500::<T>(
                    &method,
                    url,
                    "Failed to acquire a database transaction",
                    None,
                )?;
                ctx.delete::<T>(&mut tx, id.clone()).await.map_err_500::<T>(
                    &method,
                    url,
                    "Failed to delete an entity from database",
                    Some(&id),
                )?;
                tx.commit()
                    .await
                    .map_err_500::<T>(&method, url, "Failed to commit a transaction", None)?;
                Ok(Json(()))
            }

            impl<T, S> UpdateRouter<S, DbImpl> for T
            where
                T: UpdateEntity<DbImpl> + ValidationHook + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + Sync + 'static,
                <T as Entity>::Pk: DeserializeOwned + Send,
                <T as Entity>::Update: DeserializeOwned + Send,
                <T as Entity>::Replace: DeserializeOwned + Send,
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
                Path(id): Path<<T as Entity>::Pk>,
                State(state): State<S>,
                Json(input): Json<<T as Entity>::Update>,
            ) -> Result<Json<T>, (StatusCode, Json<ErrorResponse>)>
            where
                T: UpdateEntity<DbImpl> + ValidationHook + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + 'static,
                <T as Entity>::Pk: DeserializeOwned + Send,
                <T as Entity>::Update: DeserializeOwned + Send,
            {
                update_endpoint_logic(Method::PATCH, id, state, input).await
            }

            async fn replace_endpoint_impl<T, S>(
                Path(id): Path<<T as Entity>::Pk>,
                State(state): State<S>,
                Json(input): Json<<T as Entity>::Replace>,
            ) -> Result<Json<T>, (StatusCode, Json<ErrorResponse>)>
            where
                T: UpdateEntity<DbImpl> + ValidationHook + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + 'static,
                <T as Entity>::Pk: DeserializeOwned + Send,
                <T as Entity>::Replace: DeserializeOwned,
                <T as Entity>::Update: Send,
            {
                update_endpoint_logic(Method::PUT, id, state, input.into()).await
            }

            async fn update_endpoint_logic<T, S>(
                method: Method,
                id: <T as Entity>::Pk,
                state: S,
                input: <T as Entity>::Update,
            ) -> Result<Json<T>, (StatusCode, Json<ErrorResponse>)>
            where
                T: UpdateEntity<DbImpl> + ValidationHook + Routable + Serialize + Send + 'static,
                S: RouteConfig<Ctx = CtxImpl, Db = DbImpl> + Clone + Send + 'static,
                <T as Entity>::Pk: Send,
                <T as Entity>::Update: Send,
            {
                let (ctx, pool) = state.db_ctx();
                let url = <T as Routable>::entity_path();
                let mut tx =
                    pool.begin()
                        .await
                        .map_err_500::<T>(&method, url, "Failed to acquire a transaction", Some(&id))?;

                <T as ValidationHook>::before_update(&id, &input).map_err(|e| (StatusCode::BAD_REQUEST, Json(e)))?;
                let result = ctx
                    .update::<T>(&mut tx, id.clone(), input)
                    .await
                    .map_err_500::<T>(&method, url, "Failed to update an entity in database", Some(&id))?
                    .ok_or((
                        StatusCode::NOT_FOUND,
                        Json(
                            ErrorResponse::new("Not found")
                                .with_detail(&format!("An entity with id {:?} was not found.", id)),
                        ),
                    ))?;
                <T as ValidationHook>::after_update(&result).map_err(|e| (StatusCode::BAD_REQUEST, Json(e)))?;

                tx.commit()
                    .await
                    .map_err_500::<T>(&method, url, "Failed to commit a transaction", Some(&id))?;
                Ok(Json(result))
            }
        };
    }

    pub(super) use {axum_route_impl, axum_route_impl_imports};
}
