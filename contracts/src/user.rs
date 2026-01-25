use wincode::{SchemaRead, SchemaWrite};

fn host() -> String {
    std::env::var("USER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
}
fn port() -> String {
    std::env::var("USER_PORT").unwrap_or_else(|_| "4002".to_string())
}
pub fn url() -> String {
    format!("{}:{}", host(), port())
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub struct User {
    pub name: String,
    pub token: String,
}

#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub struct UserCredentials {
    pub name: String,
    pub hash: String,
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum UserRequest {
    Login(UserCredentials),
    Create(UserCredentials),
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum UserResponse {
    Valid(User),
    Invalid,
    Fault(String),
}
