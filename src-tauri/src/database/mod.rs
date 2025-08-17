//! Database layer for the finsight personal finance application.
//!
//! Provides SQLite-based persistence for financial accounts and transactions using
//! connection pooling for efficient async operations. All database interactions
//! use prepared statements for security and performance.
//!
//! # Architecture
//!
//! - **Connection Management**: SQLite connection pool with automatic file creation
//! - **Schema Management**: Automatic table creation with proper foreign key constraints
//! - **Data Types**: Integer cents for precise financial calculations, ISO 8601 dates
//! - **Error Handling**: All functions return `Result<T, sqlx::Error>` for proper error propagation
//!
//! # Entity Operations
//!
//! ## Accounts
//! - [`get_all_accounts()`] - Retrieve all financial accounts
//! - [`add_account()`] - Create new account records
//!
//! ## Transactions  
//! - [`get_transactions()`] - Query transactions for specific accounts
//! - [`add_transaction()`] - Create new transaction records with debit/credit types
//!
//! # Database Schema
//!
//! The database uses a simple relational model with accounts containing multiple
//! transactions. All monetary values are stored as integer cents to avoid
//! floating-point precision issues common in financial applications.

use sqlx::{Pool, Sqlite, SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

mod accounts;
mod migrations;
mod transactions;

pub use {accounts::*, transactions::*};

/// Initializes the SQLite database connection pool for the application.
///
/// Creates the database file if it doesn't exist, establishes a connection pool
/// for efficient async operations, and ensures all required tables are present
/// with proper schema. The connection pool enables multiple concurrent database
/// operations without blocking.
///
/// # Database Location
///
/// Creates `finsight.db` in the current working directory. For desktop applications,
/// this is typically the application's executable directory.
///
/// # Returns
/// * `Ok(SqlitePool)` - Connection pool ready for database operations
/// * `Err(sqlx::Error)` - Database initialization or table creation failure
///
/// # Errors
/// Fails if:
/// - Database file cannot be created (insufficient permissions, disk space)
/// - SQLite connection fails (file corruption, unsupported version)
/// - Table schema creation fails (SQL syntax errors, constraint violations)
/// - Connection pool setup fails (system resource limits)
///
/// # Examples
/// ```no_run
/// use crate::database;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let db_pool = database::init_db().await?;
///     
///     // Pool is now ready for all database operations
///     let accounts = database::get_all_accounts(&db_pool).await?;
///     Ok(())
/// }
/// ```
pub async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let options = SqliteConnectOptions::from_str("sqlite:./finsight.db")?.create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    // Create tables if they don't exist
    create_tables(&pool).await?;

    // Run any pending migrations
    migrations::run_migrations(&pool).await?;

    Ok(pool)
}

/// Creates all required database tables with proper schema if they don't exist.
///
/// Executes CREATE TABLE IF NOT EXISTS statements for the complete database schema.
/// Currently creates accounts and transactions tables with proper foreign key
/// relationships and constraints for financial data integrity.
///
/// # Schema Created
/// - **accounts**: Financial account records with name, type, and timestamps
/// - **transactions**: Transaction records linked to accounts with amount, type, and dates
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing table creation queries
///
/// # Returns
/// * `Ok(())` - All tables created or verified to exist
/// * `Err(sqlx::Error)` - Table creation or schema validation failure
///
/// # Errors
/// Fails if:
/// - SQL schema syntax is invalid (programming error)
/// - Database file is read-only or locked by another process
/// - Insufficient disk space for table metadata
/// - Database corruption prevents schema operations
/// - Connection pool is exhausted or disconnected
///
/// # Examples
/// ```no_run
/// use sqlx::SqlitePool;
///
/// let pool = SqlitePool::connect("sqlite:test.db").await?;
/// create_tables(&pool).await?;
/// // Database now has accounts and transactions tables ready
/// ```
async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS migrations(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            migration_name TEXT NOT NULL UNIQUE,
            applied_at TEXT DEFAULT (datetime('now'))
        )"#,
    )
    .execute(pool)
    .await?;

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

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                parent_id INTEGER,
                created_at TEXT DEFAULT (datetime('now')),
                FOREIGN KEY (parent_id) REFERENCES categories(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                account_id INTEGER NOT NULL REFERENCES accounds(id),
                amount_cents INTEGER NOT NULL,
                transaction_type TEXT NOT NULL,
                description TEXT NOT NULL,
                transaction_date TEXT NOT NULL,
                category_id INTEGER NOT NULL REFERENCES categories(id),
                created_at TEXT DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
