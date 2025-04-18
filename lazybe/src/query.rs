use crate::TableEntity;
use crate::filter::Filter;

pub trait GetQuery: TableEntity {
    fn get_query(id: Self::Pk) -> sea_query::SelectStatement;
}

pub trait ListQuery: TableEntity {
    fn list_query(filter: Filter<Self>) -> sea_query::SelectStatement;
}

pub trait CreateQuery: TableEntity {
    fn create_query(input: Self::Create) -> sea_query::InsertStatement;
}

pub trait UpdateQuery: TableEntity {
    fn update_query(id: Self::Pk, input: Self::Update) -> sea_query::UpdateStatement;
}

pub trait DeleteQuery: TableEntity {
    fn delete_query(id: Self::Pk) -> sea_query::DeleteStatement;
}
