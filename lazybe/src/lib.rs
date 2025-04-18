pub use entity::{Entity, TableEntity};

pub mod db;
pub mod entity;
pub mod filter;
pub mod page;
pub mod query;
pub mod sort;

pub mod uuid {
    pub use uuid::*;
}

pub mod macros {
    pub use lazybe_macro::*;
}

#[cfg(feature = "axum")]
pub mod router;

#[cfg(feature = "axum")]
pub mod axum {
    pub use axum::*;
}

#[cfg(feature = "oas")]
pub mod oas;
