//! Database Operations for finsight
//!
//! Handles SQLite database initialization and basic operations.

use sqlx::{Pool, Sqlite, SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

/// Initialize the SQLite database connection pool
///
/// Creates database file if it doesn't exist and returns a connection pool
pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let options = SqliteConnectOptions::from_str("sqlite:./finsight.db")?.create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    Ok(pool)
}
