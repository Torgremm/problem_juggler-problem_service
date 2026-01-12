use contracts::{ProblemResponse, UserProblem};
use sqlx::{sqlite::SqlitePoolOptions, Row};

use sqlx::{Result, SqlitePool};

pub struct UserRepository {
    pub pool: SqlitePool,
}

impl UserRepository {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }
}

impl UserRepository {
    pub async fn get(&self, id: i64) -> Result<String> {
        let row = sqlx::query("SELECT id, name, credentials FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(todo!())
    }

    pub async fn insert(&self, (data, answer): (&String, i64)) -> anyhow::Result<usize> {
        let result = sqlx::query("INSERT INTO problems (data, answer) VALUES (?,?)")
            .bind(data)
            .bind(answer)
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_rowid().try_into()?)
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

#[cfg(feature = "test-utils")]
impl UserRepository {
    pub async fn test_object() -> UserRepository {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("failed to create test db");

        sqlx::query(
            r#"
                CREATE TABLE problems (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    data TEXT NOT NULL,
                    answer INTEGER NOT NULL
                )
                "#,
        )
        .execute(&pool)
        .await
        .expect("failed to create schema");

        Self { pool }
    }
}
