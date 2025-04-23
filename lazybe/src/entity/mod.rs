use std::fmt::Debug;

/// Utilities and traits for performing CRUD operations on an Entity
pub mod ops;

/// A representation of an identifiable resource in the application domain.
pub trait Entity: Sized {
    /// Primary key
    type Pk: Debug + Clone;

    /// A type that is used to create an Entity.
    type Create;

    /// A type that is used to partially update an Entity.
    type Update;

    /// A type that is used to replace an Entity.
    /// This is a special case of partial update where all fields are updated.
    type Replace: Into<Self::Update>;

    fn entity_name() -> &'static str;
}

/// A special type of entity that is directly mapped to the database table.
pub trait TableEntity: Entity {
    type Row;
}
