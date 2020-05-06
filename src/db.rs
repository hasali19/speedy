use anyhow::Result;
use serde::Serialize;
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

use crate::speedtest;

pub struct Db(SqlitePool);

#[derive(Debug, Serialize)]
pub struct TestResult {
    id: i32,
    timestamp: String,
    ping: f32,
    download: i32,
    upload: i32,
}

impl From<speedtest::TestResult> for TestResult {
    fn from(result: speedtest::TestResult) -> Self {
        TestResult {
            id: 0,
            timestamp: result.timestamp,
            ping: result.ping.latency,
            download: result.download.bandwidth,
            upload: result.upload.bandwidth,
        }
    }
}

impl Db {
    pub async fn new(url: &str) -> Result<Db> {
        let db = Db(SqlitePool::new(url).await?);

        // Ensure database has been created.
        sqlx::query(include_str!("schema.sql"))
            .execute(&db.0)
            .await?;

        Ok(db)
    }

    #[allow(dead_code)]
    pub async fn get_all_results(&self) -> Result<Vec<TestResult>> {
        let results = sqlx::query("SELECT * FROM results")
            .map(|row: SqliteRow| TestResult {
                id: row.get(0),
                timestamp: row.get(1),
                ping: row.get(2),
                download: row.get(3),
                upload: row.get(4),
            })
            .fetch_all(&self.0)
            .await?;

        Ok(results)
    }

    pub async fn create_result(&self, result: &TestResult) -> Result<()> {
        let sql = "
        INSERT INTO results (timestamp, ping, download, upload)
        VALUES (?, ?, ?, ?)";

        sqlx::query(sql)
            .bind(&result.timestamp)
            .bind(result.ping)
            .bind(result.download)
            .bind(result.upload)
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn close(&self) {
        self.0.close().await
    }
}
