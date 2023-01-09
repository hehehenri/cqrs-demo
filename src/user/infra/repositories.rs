use crate::{user::domain::{
    entities::User,
    repositories::Repository
}, infra::uuid::Uuid};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row, postgres::PgRow};

pub struct PgRepository {
    pool: Pool<Postgres>
}

impl PgRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

fn to_user(row: PgRow) -> Result<User> {
    Ok(User::new(
        row.try_get("first_name")?,
        row.try_get("first_name")?,
    ))
}

#[async_trait]
impl Repository for PgRepository {
    async fn create(&self, user: User) -> Result<()> {
        sqlx::query("INSERT INTO users (uuid, first_name, last_name) VALUES (?, ?, ?)")
            .bind(user.uuid().to_string())
            .bind(user.first_name())
            .bind(user.last_name())
            .execute(&self.pool).await?;

        Ok(())
    }

    async fn find(&self, id: Uuid) -> Result<User> {
        let row = sqlx::query("SELECT * FROM users WHERE uuid = ?")
            .bind(id.to_string())
            .fetch_one(&self.pool).await?;

        to_user(row)
    }

    async fn list(&self) -> Result<Vec<User>> {
        let rows = sqlx::query("SELECT * FROM users")
            .fetch_all(&self.pool).await?;

        let mut users = Vec::with_capacity(rows.capacity());

        for row in rows {
            users.push(to_user(row)?)
        }

        Ok(users)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM users where id = ?")
            .bind(id.as_ref())
            .execute(&self.pool).await?;

        Ok(())
    }
}
