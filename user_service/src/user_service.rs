use crate::user_repo::UserRepoError;
use crate::user_repo::UserRepository;
use contracts::User;
use contracts::UserCredentials;
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
            Err(e) => UserResponse::Fault(e.to_string()),
        }
    }

    pub async fn login(&self, user: UserCredentials) -> UserResponse {
        match self.repo.get(&user).await {
            Ok(token) => UserResponse::Valid(User {
                name: user.name,
                token,
            }),
            Err(e) => UserResponse::Fault(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_user(name: String) -> UserCredentials {
        UserCredentials {
            name: name.clone(),
            hash: name,
        }
    }
    #[tokio::test]
    async fn login_to_five_users() {
        let repo = UserRepository::test_object().await;
        let service = UserService::new(repo);
        let users = (0..5).map(|n| get_user(format!("user{:?}", n)));

        for user in users.clone() {
            if let UserResponse::Valid(u) = service.create_user(user.clone()).await {
                assert_eq!(u.name, user.name);
            } else {
                unreachable!()
            }
        }

        for user in users {
            if let UserResponse::Valid(u) = service.login(user.clone()).await {
                assert_eq!(u.name, user.name);
            } else {
                unreachable!()
            }
        }
    }
    #[tokio::test]
    async fn create_five_users() {
        let repo = UserRepository::test_object().await;
        let service = UserService::new(repo);
        let users = (0..5).map(|n| get_user(format!("user{:?}", n)));

        for user in users.clone() {
            let result = service.create_user(user.clone()).await;
            match result {
                UserResponse::Valid(u) => assert_eq!(u.name, user.name),
                UserResponse::Invalid => println!("Invalid {:?}", result),
                UserResponse::Fault(e) => println!("{e}"),
            }
        }
        for user in users {
            assert_eq!(
                service.create_user(user.clone()).await,
                UserResponse::Invalid
            );
        }
    }
}
