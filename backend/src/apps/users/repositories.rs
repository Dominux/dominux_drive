use sqlx::Pool;

use super::models::{InUser, User};

pub struct UsersRepository<'a, T>
where
    T: sqlx::Database,
{
    pool: &'a Pool<T>,
}

impl<'a, T> UsersRepository<'a, T>
where
    T: sqlx::Database,
{
    pub fn new(pool: &'a Pool<T>) -> Self {
        Self { pool }
    }

    /// Creates new user
    pub async fn create(self, user: InUser) -> anyhow::Result<User> {
        let rec = sqlx::query("INSERT INTO users VALUES ( $1 ) RETURNING id")
            .bind(user.name)
            .fetch_one(self.pool)
            .await?;

        let user = User::from_in_user(user, rec.id);
        Ok(user)
    }
}
