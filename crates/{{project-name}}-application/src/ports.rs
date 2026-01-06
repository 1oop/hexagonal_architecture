use async_trait::async_trait;
use {{project-name}}-domain::User;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<User>>;
    async fn save(&self, user: User) -> anyhow::Result<()>;
}
