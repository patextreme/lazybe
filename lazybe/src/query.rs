use crate::filter::Filter;

pub trait GetQuery {
    type Pk;
    type Row;
    fn get_query(id: Self::Pk) -> sea_query::SelectStatement;
}

pub trait ListQuery: Sized {
    type Row;
    fn list_query(filter: Filter<Self>) -> sea_query::SelectStatement;
}

pub trait CreateQuery {
    type Create;
    type Row;
    fn create_query(input: Self::Create) -> sea_query::InsertStatement;
}

pub trait UpdateQuery {
    type Pk;
    type Update;
    type Row;
    fn update_query(id: Self::Pk, input: Self::Update) -> sea_query::UpdateStatement;
}

pub trait DeleteQuery {
    type Pk;
    fn delete_query(id: Self::Pk) -> sea_query::DeleteStatement;
}
