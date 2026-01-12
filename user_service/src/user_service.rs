use crate::user_repo::UserRepository;
use crate::user_repo::UserRow;
use anyhow::Result;
use contracts::UserRequest;
use contracts::UserResponse;

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }
}

impl UserService {
    pub async fn default() -> Self {
        Self {
            repo: UserRepository::new("sqlite:./data/problems.db")
                .await
                .expect("failed to create database"),
        }
    }
}

impl UserService {}
