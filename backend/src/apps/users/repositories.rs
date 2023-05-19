use sqlx::PgPool;

use super::models::{InUser, User};

pub struct UsersRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UsersRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Creates new user
    pub async fn create(self, user: InUser) -> anyhow::Result<User> {
        let (id,): (i64,) = sqlx::query_as("INSERT INTO users VALUES ( $1, $2 ) RETURNING id")
            .bind(&user.name)
            .bind(&user.password)
            .fetch_one(self.pool)
            .await?;

        let user = User::from_in_user(user, id as usize);
        Ok(user)
    }
}
