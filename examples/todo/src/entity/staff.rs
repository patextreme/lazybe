use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, lazybe::Newtype)]
pub struct StaffId(u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, lazybe::Entity)]
#[lazybe(table = "staff", endpoint = "/staffs", collection_api = "default")]
pub struct Staff {
    #[lazybe(primary_key)]
    pub id: StaffId,
    pub name: String,
    #[lazybe(created_at)]
    pub created_at: DateTime<Utc>,
    #[lazybe(updated_at)]
    pub updated_at: DateTime<Utc>,
}
