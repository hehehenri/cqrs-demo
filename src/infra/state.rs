use crate::user::domain::repositories::{Repository as UserRepository};
use std::sync::Arc;

pub struct State {
    repositories: Repositories
}

impl State {
    pub fn new(repositories: Repositories) -> Self {
        Self {
            repositories
        }
    }
}

pub struct Repositories {
    pub user: Arc<dyn UserRepository>
}

impl Repositories {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self {
            user: user_repository
        }
    }
}