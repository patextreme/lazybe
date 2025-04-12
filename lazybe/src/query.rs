use crate::Entity;
use crate::filter::Filter;

pub trait GetQuery: Entity {
    fn get_query(id: Self::Pk) -> sea_query::SelectStatement;
}

pub trait ListQuery: Entity + Sized {
    fn list_query(filter: Filter<Self>) -> sea_query::SelectStatement;
}

pub trait CreateQuery: Entity {
    fn create_query(input: Self::Create) -> sea_query::InsertStatement;
}

pub trait UpdateQuery: Entity {
    fn update_query(id: Self::Pk, input: Self::Update) -> sea_query::UpdateStatement;
}

pub trait DeleteQuery: Entity {
    fn delete_query(id: Self::Pk) -> sea_query::DeleteStatement;
}
