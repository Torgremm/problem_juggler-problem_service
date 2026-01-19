#![allow(warnings)]
pub mod tokens;
pub mod user_repo;
pub mod user_service;

#[cfg(feature = "test-utils")]
pub mod test_utils {

    use crate::{user_repo::UserRepository, user_service::UserService};

    pub async fn test_service() -> UserService {
        let repo = UserRepository::test_object().await;
        UserService::new(repo)
    }
}
