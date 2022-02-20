use time::OffsetDateTime;

use crate::{
    config::db::postgres::PgPool,
    dto::{LoginInput, RegisterInput, UpdateInput},
    error::{Error, Result},
    model::{CreateUserData, User},
    util::encryption,
};

pub(crate) struct AuthService;

impl AuthService {
    pub(crate) async fn sign_in(input: LoginInput, pool: &PgPool) -> Result<User> {
        let user = User::find_by_email(&input.email, pool).await?;

        if encryption::verify_password(input.password, user.password.to_owned()).await? {
            Ok(user)
        } else {
            Err(Error::WrongPassword)
        }
    }

    pub(crate) async fn sign_up(input: RegisterInput, pool: &PgPool) -> Result<User> {
        if User::find_by_name(&input.name, pool).await.is_ok() {
            return Err(Error::DuplicateUserName);
        }

        if User::find_by_email(&input.email, pool).await.is_ok() {
            return Err(Error::DuplicateUserEmail);
        }

        let data = CreateUserData {
            name: input.name,
            email: input.email,
            password: encryption::hash_password(input.password).await?,
            created_at: OffsetDateTime::now_utc().into(),
            updated_at: OffsetDateTime::now_utc().into(),
        };

        User::create(data, pool).await
    }

    pub(crate) async fn update(old: User, input: UpdateInput, pool: &PgPool) -> Result<User> {
        if !encryption::verify_password(input.old_password, old.password.to_owned()).await? {
            return Err(Error::WrongPassword);
        }

        if input.name != old.name && User::find_by_name(&input.name, pool).await.is_ok() {
            return Err(Error::DuplicateUserName);
        }

        if input.email != old.email && User::find_by_email(&input.email, pool).await.is_ok() {
            return Err(Error::DuplicateUserEmail);
        }

        let data = CreateUserData {
            name: input.name,
            email: input.email,
            password: encryption::hash_password(input.new_password).await?,
            created_at: old.created_at,
            updated_at: OffsetDateTime::now_utc().into(),
        };

        User::update(old.id, data, pool).await
    }
}
