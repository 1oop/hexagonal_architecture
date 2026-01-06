use crate::ports::UserRepository;
use {{project-name}}-domain::User;
use std::sync::Arc;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, username: String, email: String) -> anyhow::Result<User> {
        let user = User::new(username, email);
        self.repo.save(user.clone()).await?;
        Ok(user)
    }
}
