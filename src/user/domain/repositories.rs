use anyhow::Result;
use async_trait::async_trait;

use crate::infra::uuid::Uuid;
use crate::user::domain::entities::User;

#[async_trait]
pub trait Repository: Sync + Send {
    async fn create(&self, user: User) -> Result<()>;
    async fn find(&self, uuid: Uuid) -> Result<User>;
    async fn list(&self) -> Result<Vec<User>>;
    async fn delete(&self, user: Uuid) -> Result<()>;
}
