use contracts::user::UserCredentials;
use sqlx::sqlite::SqlitePoolOptions;

use sqlx::{Result, SqlitePool};

pub struct UserRepository {
    pub pool: SqlitePool,
}

impl UserRepository {
    pub async fn new(database_url: &str) -> Result<Self> {
        #[cfg(feature = "test-utils")]
        if database_url == "sqlite::memory:" {
            return Ok(Self::test_object().await);
        }
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }
}

impl UserRepository {
    pub async fn create_user(&self, user: &UserCredentials) -> Result<UserRow, UserRepoError> {
        let row = sqlx::query("SELECT id, name FROM users WHERE name = ?")
            .bind(&user.name)
            .fetch_optional(&self.pool)
            .await?;

        if row.is_some() {
            return Err(UserRepoError::DuplicateName);
        }

        let id = self.insert(user).await?;

        Ok(UserRow {
            id,
            name: user.name.clone(),
            credentials: user.hash.clone(),
        })
    }
    pub async fn get(&self, user: &UserCredentials) -> Result<String> {
        let _row =
            sqlx::query("SELECT id, name, credentials FROM users WHERE (name,credentials) = (?,?)")
                .bind(user.name.clone())
                .bind(user.hash.clone())
                .fetch_one(&self.pool)
                .await?;

        Ok("aaaaaaaa".to_string())
    }

    async fn insert(&self, user: &UserCredentials) -> Result<i64, UserRepoError> {
        let result = sqlx::query("INSERT INTO users (name, credentials) VALUES (?,?)")
            .bind(user.name.clone())
            .bind(user.hash.clone())
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_rowid())
    }
}

#[derive(Debug)]
pub struct UserRow {
    pub id: i64,
    pub name: String,
    credentials: String,
}

impl UserRow {
    pub fn new(id: i64, name: String, credentials: String) -> Self {
        Self {
            id,
            name,
            credentials,
        }
    }
}

#[derive(Debug)]
pub enum UserRepoError {
    DuplicateName,
    Sqlx(sqlx::Error),
}
impl std::fmt::Display for UserRepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRepoError::DuplicateName => write!(f, "user with that name already exists"),
            UserRepoError::Sqlx(e) => write!(f, "database error: {}", e),
        }
    }
}

impl std::error::Error for UserRepoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UserRepoError::Sqlx(e) => Some(e),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for UserRepoError {
    fn from(err: sqlx::Error) -> Self {
        UserRepoError::Sqlx(err)
    }
}

impl UserRepository {
    pub async fn test_object() -> UserRepository {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("failed to create test db");

        sqlx::query(
            r#"
                CREATE TABLE users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    credentials TEXT NOT NULL
                )
                "#,
        )
        .execute(&pool)
        .await
        .expect("failed to create schema");

        Self { pool }
    }
}

#[cfg(test)]
impl UserRow {
    pub fn test_row(num: i64) -> Self {
        Self {
            id: num,
            name: format!("user{}", num),
            credentials: "verystrongand-secretpasswoerd!!!9".to_string(),
        }
    }
}
