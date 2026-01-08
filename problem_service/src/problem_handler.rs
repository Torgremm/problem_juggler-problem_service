use sqlx::{sqlite::SqlitePoolOptions, Row};
use std::pin::Pin;

use sqlx::{Result, SqlitePool};

use crate::test_template::Test;

pub struct ProblemRepository {
    pub pool: SqlitePool,
}

impl ProblemRepository {
    pub async fn new(database_url: &str) -> Result<Self> {
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

    pub async fn insert(&self, (data, answer): (String, i64)) -> anyhow::Result<usize> {
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
    pub fn validate(&self, answer: i64) -> bool {
        answer == self.answer
    }
}
impl Test for ProblemRepository {
    fn test_object() -> Pin<Box<dyn Future<Output = Self> + Send>> {
        Box::pin(async move {
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
        })
    }
}
