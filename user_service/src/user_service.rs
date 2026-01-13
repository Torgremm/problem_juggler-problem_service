use crate::user_repo::UserRepoError;
use crate::user_repo::UserRepository;
use crate::user_repo::UserRow;
use anyhow::Result;
use contracts::User;
use contracts::UserCredentials;
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
            repo: UserRepository::new("sqlite:./data/users.db")
                .await
                .expect("failed to create database"),
        }
    }
}

impl UserService {
    pub async fn create_user(&self, user: UserCredentials) -> UserResponse {
        match self.repo.create_user(&user).await {
            Ok(()) => {
                let token = "aaaaaaaa".to_string();
                UserResponse::Valid(User {
                    name: user.name,
                    token,
                })
            }
            Err(UserRepoError::DuplicateName) => UserResponse::Invalid,
            _ => UserResponse::Fault,
        }
    }

    pub async fn login(&self, user: UserCredentials) -> UserResponse {
        match self.repo.get(&user).await {
            Ok(token) => UserResponse::Valid(User {
                name: user.name,
                token,
            }),
            Err(e) => UserResponse::Fault,
        }
    }
}
