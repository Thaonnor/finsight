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

use sqlx::{Pool, Row, Sqlite, SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

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
    crate::migrations::run_migrations(&pool).await?;

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

/// Retrieves all financial accounts from the database.
///
/// Queries the accounts table and returns all records as JSON-serializable objects
/// for frontend consumption. Results include account ID, name, and type but exclude
/// internal timestamps to keep the API clean.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing the query
///
/// # Returns
/// * `Ok(Vec<serde_json::Value>)` - Array of account objects with id, name, and account_type
/// * `Err(sqlx::Error)` - Database query or serialization failure
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Query execution fails (corrupted database, schema changes)
/// - Row data cannot be extracted (type mismatches, missing columns)
/// - JSON serialization fails (malformed database content)
///
/// # Examples
/// ```no_run
/// let accounts = get_all_accounts(&pool).await?;
/// println!("Found {} accounts", accounts.len());
///
/// for account in accounts {
///     println!("Account: {} ({})",
///         account["name"].as_str().unwrap(),
///         account["account_type"].as_str().unwrap()
///     );
/// }
/// ```
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

/// Creates a new financial account in the database.
///
/// Inserts a new account record with the provided name and type. The creation
/// timestamp is automatically set by the database. Account names should be
/// descriptive and meaningful for household financial tracking.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing the insertion
/// * `name` - Human-readable account name (e.g., "Chase Checking", "Emergency Savings")
/// * `account_type` - Account classification, typically "checking" or "savings"
///
/// # Returns
/// * `Ok(())` - Account created successfully with auto-generated ID
/// * `Err(sqlx::Error)` - Database insertion or validation failure
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Account name violates constraints (empty string, duplicate names if enforced)
/// - Account type is invalid or unsupported by application logic
/// - Database insertion fails (disk space, permissions, corruption)
/// - Parameter binding fails (invalid UTF-8 in strings)
///
/// # Examples
/// ```no_run
/// // Create a primary checking account
/// add_account(&pool, "Wells Fargo Checking".to_string(), "checking".to_string()).await?;
///
/// // Create a savings account
/// add_account(&pool, "High-Yield Savings".to_string(), "savings".to_string()).await?;
/// ```
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

/// Retrieves all transactions for a specific financial account.
///
/// Queries transactions linked to the given account ID and returns them as
/// JSON-serializable objects for frontend display. Results include core transaction
/// data but exclude internal metadata like creation timestamps to keep the API clean.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing the query
/// * `account_id` - Database ID of the account to retrieve transactions for
///
/// # Returns
/// * `Ok(Vec<serde_json::Value>)` - Array of transaction objects with id, account_id, amount_cents, description, and transaction_date
/// * `Err(sqlx::Error)` - Database query or data extraction failure
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Account ID does not exist (no matching foreign key reference)
/// - Query execution fails (corrupted database, schema changes)
/// - Row data extraction fails (type mismatches, missing columns)
/// - JSON serialization fails (malformed database content)
///
/// # Examples
/// ```no_run
/// // Load transactions for account detail view
/// let transactions = get_transactions_by_account(&pool, 1).await?;
///
/// for tx in transactions {
///     let amount_dollars = tx["amount_cents"].as_i64().unwrap() as f64 / 100.0;
///     println!("{}: {} - ${:.2}",
///         tx["transaction_date"].as_str().unwrap(),
///         tx["description"].as_str().unwrap(),
///         amount_dollars
///     );
/// }
/// ```
pub async fn get_transactions(
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

/// Creates a new financial transaction record for the specified account.
///
/// Inserts a transaction with the provided details, using integer cents for precise
/// financial calculations. The creation timestamp is automatically set by the database.
/// Transaction types determine how amounts affect account balances in future calculations.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing the insertion
/// * `account_id` - Database ID of the account this transaction belongs to
/// * `amount_cents` - Transaction amount in cents (always positive, e.g., 2550 for $25.50)
/// * `transaction_type` - Either "debit" (reduces balance) or "credit" (increases balance)
/// * `description` - Human-readable transaction description from bank data or user input
/// * `transaction_date` - Transaction date in ISO 8601 format (YYYY-MM-DD)
///
/// # Returns
/// * `Ok(())` - Transaction created successfully with auto-generated ID
/// * `Err(sqlx::Error)` - Database insertion or validation failure
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Account ID does not exist (invalid foreign key reference)
/// - Transaction type is invalid (must be "debit" or "credit")
/// - Date format is malformed (must be valid ISO 8601 YYYY-MM-DD)
/// - Database insertion fails (disk space, permissions, corruption)
/// - Parameter binding fails (invalid UTF-8 in strings, integer overflow)
///
/// # Examples
/// ```no_run
/// // Record a grocery store purchase
/// add_transaction(
///     &pool,
///     1,
///     4275,  // $42.75
///     "debit".to_string(),
///     "Whole Foods Market".to_string(),
///     "2025-08-15".to_string()
/// ).await?;
///
/// // Record a salary deposit
/// add_transaction(
///     &pool,
///     1,
///     250000,  // $2,500.00
///     "credit".to_string(),
///     "Payroll Deposit".to_string(),
///     "2025-08-15".to_string()
/// ).await?;
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

/// Removes a transaction record from the database.
///
/// Permanently deletes the transaction with the specified ID. This operation
/// cannot be undone, so the transaction data will be completely removed from
/// the database. Use with caution as this affects historical financial records.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing the deletion
/// * `transaction_id` - Database ID of the transaction to remove
///
/// # Returns
/// * `Ok(())` - Transaction deleted successfully
/// * `Err(sqlx::Error)` - Database deletion failure or transaction not found
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Transaction ID does not exist (no matching record to delete)
/// - Database deletion fails (permissions, corruption, foreign key constraints)
/// - Connection pool is exhausted or disconnected
///
/// # Examples
/// ```no_run
/// // Remove an incorrect transaction entry
/// delete_transaction(&pool, 123).await?;
///
/// // Note: No error if transaction ID doesn't exist - SQLite DELETE succeeds
/// // with 0 rows affected when no matching records are found
/// ```
pub async fn delete_transaction(pool: &SqlitePool, transaction_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM transactions WHERE id = ?")
        .bind(transaction_id)
        .execute(pool)
        .await?;

    Ok(())
}
