use std::time::SystemTime;

use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    #[serde(skip_serializing)]
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) email: String,
    #[serde(skip_serializing)]
    pub(crate) password: String,
    #[serde(skip_serializing)]
    pub(crate) created_at: SystemTime,
    #[serde(skip_serializing)]
    pub(crate) updated_at: SystemTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub(crate) struct CreateUserData {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) created_at: SystemTime,
    pub(crate) updated_at: SystemTime,
}

#[derive(Serialize)]
pub(crate) struct Health {
    pub(crate) db: DbHealth,
}

#[derive(Serialize)]
pub(crate) enum DbHealth {
    Available,

    Unavailable,
}
