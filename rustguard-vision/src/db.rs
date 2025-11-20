use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Pool, Postgres, Row};
use anyhow::Result;

#[derive(Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn record_visit(&self, name: &str, confidence: f32) -> Result<i64> {
        // Using runtime query to avoid compile-time DB check requirement in sandbox
        let row = sqlx::query(
            r#"
            INSERT INTO visits (visitor_name, confidence, visited_at)
            VALUES ($1, $2, NOW())
            RETURNING id
            "#
        )
        .bind(name)
        .bind(confidence)
        .fetch_one(&self.pool)
        .await?;

        let id: i64 = row.try_get("id")?;
        Ok(id)
    }
}
