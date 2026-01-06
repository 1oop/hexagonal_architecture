use async_trait::async_trait;
use {{project-name}}-application::UserRepository;
use {{project-name}}-domain::User;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub struct InMemoryUserRepository {
    users: Mutex<HashMap<Uuid, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<User>> {
        let users = self.users.lock().unwrap();
        Ok(users.get(&id).cloned())
    }

    async fn save(&self, user: User) -> anyhow::Result<()> {
        let mut users = self.users.lock().unwrap();
        users.insert(user.id, user);
        Ok(())
    }
}
