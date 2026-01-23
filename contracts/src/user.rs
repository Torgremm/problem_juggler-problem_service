use wincode::{SchemaRead, SchemaWrite};

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
