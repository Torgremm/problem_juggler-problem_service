use contracts::problem::ValidationResponse;
use contracts::problem::{ProblemResponse, UserProblem};
use sqlx::{sqlite::SqlitePoolOptions, Row};

use sqlx::{Result, SqlitePool};

pub struct ProblemRepository {
    pub pool: SqlitePool,
}

impl ProblemRepository {
    pub async fn new(database_url: &str) -> Result<Self> {
        #[cfg(feature = "test-utils")]
        if database_url == "sqlite::memory:" {
            return Ok(Self::test_object().await);
        }
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }
}

impl ProblemRepository {
    pub async fn get(&self, id: i64) -> Result<ProblemRow> {
        let row = sqlx::query("SELECT id, data, answer FROM problems WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(ProblemRow {
            id: row.try_get("id")?,
            data: row.try_get("data")?,
            answer: row.try_get("answer")?,
        })
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
pub struct ProblemRow {
    pub id: i64,
    pub data: String,
    pub answer: i64,
}

impl ProblemRow {
    pub fn new(id: i64, answer: i64, data: String) -> Self {
        Self { id, data, answer }
    }
    pub fn validate(&self, answer: i64) -> ValidationResponse {
        if answer == self.answer {
            return ValidationResponse::Valid;
        }
        if answer > self.answer {
            return ValidationResponse::Higher;
        }

        ValidationResponse::Lower
    }
    pub fn to_response(&self) -> ProblemResponse {
        let p = UserProblem {
            id: self.id,
            data: self.data.clone(),
        };
        ProblemResponse::Ok(p)
    }
}

#[cfg(feature = "test-utils")]
impl ProblemRepository {
    pub async fn test_object() -> ProblemRepository {
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
