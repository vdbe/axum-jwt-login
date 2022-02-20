use crate::{
    config::db::postgres::PgPool,
    error::Result,
    model::{DbHealth, Health},
};
pub(crate) struct HealthService;

impl HealthService {
    pub(crate) async fn get(pool: &PgPool) -> Result<Health> {
        Ok(Health {
            db: DbHealth::check(pool).await,
        })
    }
}
