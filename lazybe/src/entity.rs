use std::fmt::Debug;

pub trait Entity {
    type Row;
    type Pk: Debug + Clone;
    type Create;
    type Update;
    type Replace: Into<Self::Update>;

    fn entity_name() -> &'static str;
}
