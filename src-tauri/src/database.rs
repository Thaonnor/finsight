//! Database Operations for finsight
//!
//! Handles SQLite database initialization and basic operations.

use sqlx::{Pool, Sqlite, SqlitePool, sqlite::SqliteConnectOptions, Row};
use std::str::FromStr;

/// Initialize the SQLite database connection pool
///
/// Creates a new SQLite database file if it doesn't exist and establishes
/// a connection pool for async database operations. Also ensures all required
/// tables are created with the proper schema.
///
/// The database file is created in the current working directory as `finsight.db`
///
/// # Returns
///
/// * `Ok(Pool<Sqlite>)`    - A SQLite connection pool ready for queries
/// * `Err(sqlxx::Error)`   - Database initialization error
///
/// # Errors
///
/// This function will return an error if:
/// * The database file cannot be created (permissions, disk space)
/// * The database connection fails (file corruption, unsupported SQLite version)
/// * Table creation queries fail (syntax errors, constraint violations)
///
/// # Examples
///
/// ```no_run
/// use finsight::database;
///
/// #[tokio::main]
/// async fn main() {
///     let pool = database::init_db().await
///         .expect("Failed to initialize database.");
///
///     // Use pool for database operations...
/// }
/// ```
pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let options = SqliteConnectOptions::from_str("sqlite:./finsight.db")?.create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    // Create tables if they don't exist
    create_tables(&pool).await?;

    Ok(pool)
}

/// Creates database tables if they don't exist
///
/// Ensures all required tables are present in the database by creating them
/// with proper schemas if they don't already exist. Currently creates the
/// accounts table for storing user financial accounts.
///
/// # Arguments
///
/// * `pool` - A reference to the SQLite connection pool
///
/// # Returns
///
/// * `Ok(())` - All tables created successfully (unit type means "success, no data")
/// * `Err(sqlx::Error)` - Table creation failed
///
/// # Errors
///
/// This function will return an error if:
/// * SQL syntax in table creation queries is invalid
/// * Database file is read-only or corrupted
/// * Insufficient disk space for table creation
/// * Database connection is lost during table creation
///
/// # Examples
///
/// ```no_run
/// let pool = SqlitePool::connect("sqlite::test.db").await?;
/// create_tables(&pool).await?;
/// ```
async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS accounts (
                id INTEGER PRIMARY KEY AUTOINCREMENT, 
                name TEXT NOT NULL, 
                account_type TEXT NOT NULL, 
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all accounts from the database
///
/// # Arguments
/// * `pool` - Database connection pool reference
///
/// # Returns
/// * `Ok(Vec<serde_json::Value>)` - List of accounts as JSON objects
/// * `Err(sqlx::Error)` - Database query error
pub async fn get_all_accounts(pool: &SqlitePool) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let accounts = sqlx::query("SELECT id, name, account_type, created_at FROM accounts")
        .fetch_all(pool)
        .await?;

    let result: Vec<serde_json::Value> = accounts
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<i64, _>("id"),
                "name": row.get::<String, _>("name"),
                "account_type": row.get::<String, _>("account_type")
            })
        })
        .collect();

    Ok(result)
}
