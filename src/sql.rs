use crate::config::db::postgres::PgPool;
use crate::diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    error::Result,
    model::{CreateUserData, DbHealth, User},
    schema::users,
};

impl User {
    pub(crate) async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(users::table.find(id).first(&conn)?)
    }

    pub(crate) async fn find_by_email(email: &str, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(users::table.filter(users::email.eq(email)).first(&conn)?)
    }

    pub(crate) async fn find_by_name(name: &str, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(users::table.filter(users::name.eq(name)).first(&conn)?)
    }

    pub(crate) async fn create(data: CreateUserData, pool: &PgPool) -> Result<User> {
        let conn = pool.get()?;

        Ok(diesel::insert_into(users::table)
            .values(&data)
            .returning(users::all_columns)
            .get_result(&conn)?)
    }

    pub(crate) async fn update(id: Uuid, data: CreateUserData, pool: &PgPool) -> Result<User> {
        let conn = pool.get()?;

        Ok(diesel::update(users::table.find(id))
            .set((
                users::name.eq(data.name),
                users::email.eq(data.email),
                users::password.eq(data.password),
                users::created_at.eq(data.created_at),
                users::updated_at.eq(data.updated_at),
            ))
            .get_result(&conn)?)
    }
}

impl DbHealth {
    pub(crate) async fn check(pool: &PgPool) -> Self {
        match pool.get() {
            Ok(_) => Self::Available,
            Err(_) => Self::Unavailable,
        }
    }
}
