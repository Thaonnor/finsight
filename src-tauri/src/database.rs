//! Database Operations for finsight
//!
//! Handles SQLite database initialization and basic operations.

use sqlx::{Pool, Row, Sqlite, SqlitePool, sqlite::SqliteConnectOptions};
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

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                account_id INTEGER NOT NULL,
                amount_cents INTEGER NOT NULL,
                transaction_type TEXT NOT NULL,
                description TEXT NOT NULL,
                transaction_date TEXT NOT NULL,
                created_at TEXT DEFAULT (datetime('now')),
                FOREIGN KEY (account_id) REFERENCES accounts(id)
        )
        "#,
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

/// Adds a new account to the database.
///
/// Creates a new account record with the provided name and type. The `created_at` timestamp is automatically set by the database using CURRENT_TIMESTAMP.
///
/// # Arguments
///
/// * `pool` - A reference to the SQLite connection pool
/// * `name` - The display name for the account (e.g., "Chase Checking")
/// * `account_type` - The type of account ("checking" or "savings")
///
/// # Returns
///
/// Returns `Ok(())` on successful insertion, or an `sqlx::Error` if the database operation fails.
pub async fn add_account(
    pool: &SqlitePool,
    name: String,
    account_type: String,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO accounts (name, account_type) VALUES (?, ?)")
        .bind(name)
        .bind(account_type)
        .execute(pool)
        .await?;

    Ok(())
}

/// Fetch all transactions for a specific account
///
/// # Arguments
/// * `pool` - Database connection pool reference
/// * `account_id` - The ID of the account to get transactions for
///
/// # Returns
/// * `Ok(Vec<serde_json::Value>)` - List of transactions as JSON objects
/// * `Err(sqlx::Error)` - Database query error
pub async fn get_transactions_by_account(
    pool: &SqlitePool,
    account_id: i64,
) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let transactions = sqlx::query("SELECT id, account_id, amount_cents, description, transaction_date FROM transactions WHERE account_id = ?").bind(account_id).fetch_all(pool).await?;

    let result: Vec<serde_json::Value> = transactions
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<i64, _>("id"),
                "account_id": row.get::<i64, _>("account_id"),
                "amount_cents": row.get::<i64, _>("amount_cents"),
                "description": row.get::<String, _>("description"),
                "transaction_date": row.get::<String, _>("transaction_date")
            })
        })
        .collect();

    Ok(result)
}

/// Adds a new transaction to the database
///
/// Creates a new transaction record linked to the specific account. The `created_at`
/// timestamp is automatically set by the database using SQLite's datetime ('now').
///
/// # Arguments
///
/// * `pool` - A reference to the SQLite connection pool
/// * `account_id` - The ID of the account this transaction belongs to
/// * `amount_cents` - The transaction amount in cents
/// * `transaction_type` - The type of transaction (credit or debit)
/// * `description` - A description of the transaction (e.g., "Grocery store purchase")
/// * `transaction_date` - The date the transaction occurred in ISO 8601 format (YYYY-MM-DD)
///
/// # Returns
///
/// Returns `Ok(())` on succesful insertion, or an `sqlx::Error` if the database operation failed
///
/// # Examples
///
/// ```no_run
/// // Add a $25.50 debit transaction
/// add_transaction(&pool, 1, 2550, debit, "Coffee shop".to_string(), "2025-08-13".to_string()).await?;
/// ```
pub async fn add_transaction(
    pool: &SqlitePool,
    account_id: i64,
    amount_cents: i64,
    transaction_type: String,
    description: String,
    transaction_date: String,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO transactions (
        account_id,
        amount_cents,
        transaction_type,
        description,
        transaction_date) 
        VALUES (?, ?, ?, ?, ?)
    "#,
    )
    .bind(account_id)
    .bind(amount_cents)
    .bind(transaction_type)
    .bind(description)
    .bind(transaction_date)
    .execute(pool)
    .await?;

    Ok(())
}
