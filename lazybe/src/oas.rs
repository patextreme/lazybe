use axum::http::StatusCode;
use utoipa::openapi::path::{Operation, OperationBuilder, Parameter, ParameterIn};
use utoipa::openapi::request_body::RequestBody;
use utoipa::openapi::{Components, Content, HttpMethod, OpenApi, OpenApiBuilder, PathItem, Paths, Response};
use utoipa::{PartialSchema, ToSchema};

use crate::router::{EntityCollectionApi, ErrorResponse, Routable};
use crate::{CreateQuery, DeleteQuery, Entity, GetQuery, ListQuery, UpdateQuery};

const APPLICATION_JSON: &str = "application/json";

pub trait GetRouterDoc {
    fn get_endpoint_doc() -> OpenApi;
}

pub trait ListRouterDoc {
    fn list_endpoint_doc() -> OpenApi;
}

pub trait CreateRouterDoc {
    fn create_endpoint_doc() -> OpenApi;
}

pub trait UpdateRouterDoc {
    fn update_endpoint_doc() -> OpenApi;
    fn replace_endpoint_doc() -> OpenApi;
}

pub trait DeleteRouterDoc {
    fn delete_endpoint_doc() -> OpenApi;
}

impl<T> GetRouterDoc for T
where
    T: GetQuery + Routable + ToSchema,
{
    fn get_endpoint_doc() -> OpenApi {
        let operation = Operation::builder()
            .summary(Some(format!("Get {} entity by ID", <T as ToSchema>::name())))
            .parameter(Parameter::new("id"))
            .json_response::<T>(StatusCode::OK, "Entity retrieved successfully")
            .error_response(StatusCode::BAD_REQUEST)
            .error_response(StatusCode::NOT_FOUND)
            .error_response(StatusCode::INTERNAL_SERVER_ERROR)
            .build();

        let path = <T as Routable>::entity_path();
        let paths = Paths::builder()
            .path(path, PathItem::new(HttpMethod::Get, operation))
            .build();

        let components = {
            let mut schemas = Vec::new();
            <T as ToSchema>::schemas(&mut schemas);
            Components::builder().schemas_from_iter(schemas).build()
        };

        OpenApiBuilder::new().paths(paths).components(Some(components)).build()
    }
}

impl<T> ListRouterDoc for T
where
    T: ListQuery + EntityCollectionApi + Routable + ToSchema,
    <T as EntityCollectionApi>::Resp: ToSchema,
    <T as EntityCollectionApi>::Query: ToSchema,
{
    fn list_endpoint_doc() -> OpenApi {
        let operation = Operation::builder()
            .summary(Some(format!("List {} entity", <T as ToSchema>::name())))
            .query_object_param::<<T as EntityCollectionApi>::Query>()
            .json_response::<<T as EntityCollectionApi>::Resp>(StatusCode::OK, "Entities retrieved successfully")
            .error_response(StatusCode::BAD_REQUEST)
            .error_response(StatusCode::INTERNAL_SERVER_ERROR)
            .build();

        let path = <T as Routable>::entity_collection_path();
        let paths = Paths::builder()
            .path(path, PathItem::new(HttpMethod::Get, operation))
            .build();

        let components = {
            let mut schemas = Vec::new();
            <<T as EntityCollectionApi>::Resp as ToSchema>::schemas(&mut schemas);
            <<T as EntityCollectionApi>::Query as ToSchema>::schemas(&mut schemas);
            Components::builder()
                .schemas_from_iter(schemas)
                .schema(<T as ToSchema>::name(), <T as PartialSchema>::schema())
                .build()
        };

        OpenApiBuilder::new().paths(paths).components(Some(components)).build()
    }
}

impl<T> CreateRouterDoc for T
where
    T: CreateQuery + Routable + ToSchema,
    <T as Entity>::Create: ToSchema,
{
    fn create_endpoint_doc() -> OpenApi {
        let operation = Operation::builder()
            .summary(Some(format!("Create a new {} entity", <T as ToSchema>::name())))
            .json_request::<<T as Entity>::Create>()
            .json_response::<T>(StatusCode::CREATED, "Entity created successfully")
            .error_response(StatusCode::BAD_REQUEST)
            .error_response(StatusCode::INTERNAL_SERVER_ERROR)
            .build();

        let path = <T as Routable>::entity_collection_path();
        let paths = Paths::builder()
            .path(path, PathItem::new(HttpMethod::Post, operation))
            .build();

        let components = {
            let mut schemas = Vec::new();
            <<T as Entity>::Create as ToSchema>::schemas(&mut schemas);
            Components::builder().schemas_from_iter(schemas).build()
        };

        OpenApiBuilder::new().paths(paths).components(Some(components)).build()
    }
}

impl<T> UpdateRouterDoc for T
where
    T: UpdateQuery + Routable + ToSchema,
    <T as Entity>::Update: ToSchema,
    <T as Entity>::Replace: ToSchema,
{
    fn update_endpoint_doc() -> OpenApi {
        let operation = Operation::builder()
            .summary(Some(format!(
                "Parital update an existing {} entity",
                <T as ToSchema>::name()
            )))
            .parameter(Parameter::new("id"))
            .json_request::<<T as Entity>::Update>()
            .json_response::<T>(StatusCode::OK, "Entity updated successfully")
            .error_response(StatusCode::BAD_REQUEST)
            .error_response(StatusCode::NOT_FOUND)
            .error_response(StatusCode::INTERNAL_SERVER_ERROR)
            .build();

        let path = <T as Routable>::entity_path();
        let paths = Paths::builder()
            .path(path, PathItem::new(HttpMethod::Patch, operation))
            .build();

        let components = {
            let mut schemas = Vec::new();
            <<T as Entity>::Update as ToSchema>::schemas(&mut schemas);
            Components::builder().schemas_from_iter(schemas).build()
        };

        OpenApiBuilder::new().paths(paths).components(Some(components)).build()
    }

    fn replace_endpoint_doc() -> OpenApi {
        let operation = Operation::builder()
            .summary(Some(format!("Replace an existing {}", <T as ToSchema>::name())))
            .parameter(Parameter::new("id"))
            .json_request::<<T as Entity>::Replace>()
            .json_response::<T>(StatusCode::OK, "Entity replaced successfully")
            .error_response(StatusCode::BAD_REQUEST)
            .error_response(StatusCode::NOT_FOUND)
            .error_response(StatusCode::INTERNAL_SERVER_ERROR)
            .build();

        let path = <T as Routable>::entity_path();
        let paths = Paths::builder()
            .path(path, PathItem::new(HttpMethod::Put, operation))
            .build();

        let components = {
            let mut schemas = Vec::new();
            <<T as Entity>::Replace as ToSchema>::schemas(&mut schemas);
            Components::builder().schemas_from_iter(schemas).build()
        };

        OpenApiBuilder::new().paths(paths).components(Some(components)).build()
    }
}

impl<T> DeleteRouterDoc for T
where
    T: DeleteQuery + Routable + ToSchema,
{
    fn delete_endpoint_doc() -> OpenApi {
        let operation = Operation::builder()
            .summary(Some(format!("Delete {} entity by ID", <T as ToSchema>::name())))
            .parameter(Parameter::new("id"))
            .response("200", Response::new("Entity deleted successfully"))
            .error_response(StatusCode::BAD_REQUEST)
            .error_response(StatusCode::INTERNAL_SERVER_ERROR)
            .build();

        let path = <T as Routable>::entity_path();
        let paths = Paths::builder()
            .path(path, PathItem::new(HttpMethod::Delete, operation))
            .build();

        OpenApiBuilder::new().paths(paths).build()
    }
}

trait OperationBuilderExt {
    fn query_object_param<T: ToSchema>(self) -> OperationBuilder;
    fn json_request<T: ToSchema>(self) -> OperationBuilder;
    fn json_response<T: ToSchema>(self, status: StatusCode, desc: &str) -> OperationBuilder;
    fn error_response(self, status: StatusCode) -> OperationBuilder;
}

impl OperationBuilderExt for OperationBuilder {
    fn query_object_param<T: ToSchema>(self) -> OperationBuilder {
        let schema = <T as PartialSchema>::schema();
        let mut result = self;
        result = result.parameter(
            Parameter::builder()
                .parameter_in(ParameterIn::Query)
                .name("query_object")
                .schema(Some(schema))
                .build(),
        );
        result
    }

    fn json_request<T: ToSchema>(self) -> OperationBuilder {
        self.request_body(Some(
            RequestBody::builder()
                .content(APPLICATION_JSON, Content::new(Some(<T as PartialSchema>::schema())))
                .build(),
        ))
    }

    fn json_response<T: PartialSchema>(self, status: StatusCode, desc: &str) -> OperationBuilder {
        self.response(
            status.as_str(),
            Response::builder()
                .description(desc)
                .content(APPLICATION_JSON, Content::new(Some(<T as PartialSchema>::schema())))
                .build(),
        )
    }

    fn error_response(self, status: StatusCode) -> OperationBuilder {
        self.response(
            status.as_str(),
            Response::builder()
                .content(APPLICATION_JSON, Content::new(Some(ErrorResponse::schema())))
                .build(),
        )
    }
}
