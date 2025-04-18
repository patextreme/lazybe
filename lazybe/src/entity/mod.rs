use std::fmt::Debug;

pub mod ops;

pub trait Entity: Sized {
    type Pk: Debug + Clone;
    type Create;
    type Update;
    type Replace: Into<Self::Update>;

    fn entity_name() -> &'static str;
}

pub trait TableEntity: Entity {
    type Row;
}
