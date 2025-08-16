// Prevents console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! # finsight
//!
//! A desktop personal finance application that transforms household financial data
//! into business-style financial statements and insights.
//!
//! Built with Tauri for cross-platform desktop deployment, Vue.js for the frontend,
//! and SQLite for local data storage.
//!
//! ## Architecture
//!
//! - **Frontend**: Vue.js with Composition API
//! - **Backend**: Rust with Tauri commands for database operations
//! - **Database**: SQLite with manual transaction and account management

mod database;
mod migrations;
use sqlx::SqlitePool;

/// Application entry point for the finsight personal finance desktop application.
///
/// Initializes the SQLite database connection pool, configures the Tauri runtime
/// with command handlers, and starts the desktop application event loop.
///
/// # Database Initialization
///
/// Creates a SQLite connection pool using the configured database URL. The database
/// schema is automatically created if it doesn't exist.
///
/// # Command Registration
///
/// Registers the following Tauri command handlers for frontend-backend communication:
/// - `get_accounts` - Retrieves all financial accounts
/// - `add_account` - Creates a new financial account  
/// - `get_transactions` - Fetches transactions for a specific account
/// - `add_transaction` - Creates a new transaction record
///
/// # Runtime Behavior
///
/// On Windows release builds, runs without a console window. In debug builds,
/// a console is available for logging and debugging.
///
/// # Errors
///
/// Returns an error if:
/// - Database initialization fails (I/O errors, permissions, disk space)
/// - Tauri context generation fails (build configuration issues)
/// - Application startup fails (missing system dependencies, display server issues)
///
/// # Panics
///
/// Panics if the Tauri application fails to run after successful initialization.
/// This typically indicates critical system-level issues that cannot be recovered from.
///
/// # Examples
///
/// ```no_run
/// // Entry point is called automatically by the Rust runtime
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Application initialization and startup...
/// }
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the database
    let pool = database::init_db().await?;

    tauri::Builder::default()
        .manage(pool)
        .invoke_handler(tauri::generate_handler![
            get_accounts,
            add_account,
            get_transactions,
            add_transaction
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    Ok(())
}

/// Retrieves all financial accounts from the database.
///
/// Returns account records as JSON-serializable values for frontend consumption.
/// Each account includes id, name, account type, and creation timestamp.
///
/// # Arguments
/// * `db` - SQLite connection pool managed by Tauri state
///
/// # Returns
/// * `Ok(Vec<serde_json::Value>)` - Array of account objects
/// * `Err(String)` - Database error message for frontend display
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Query execution fails (corrupted database, schema mismatch)
/// - JSON serialization fails (malformed data in database)
///
/// # Examples
/// ```javascript
/// // Retrieve all accounts for display in accounts list
/// const accounts = await invoke('get_accounts');
/// accounts.forEach(account => {
///     console.log(`${account.name} (${account.account_type})`);
/// });
/// ```
#[tauri::command]
async fn get_accounts(db: tauri::State<'_, SqlitePool>) -> Result<Vec<serde_json::Value>, String> {
    database::get_all_accounts(&*db)
        .await
        .map_err(|e| e.to_string())
}

/// Creates a new financial account in the database.
///
/// Validates input parameters and inserts a new account record with the current
/// timestamp. Account names should be descriptive and unique within the household.
///
/// # Arguments
/// * `db` - SQLite connection pool managed by Tauri state
/// * `name` - Human-readable account name (e.g., "Chase Checking", "Emergency Savings")
/// * `account_type` - Account classification, must be "checking" or "savings"
///
/// # Returns
/// * `Ok(())` - Account created successfully
/// * `Err(String)` - Validation or database error message for frontend display
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Account name already exists (unique constraint violation)
/// - Invalid account type provided (must be "checking" or "savings")
/// - Database insertion fails (disk space, permissions, corruption)
///
/// # Examples
/// ```javascript
/// // Create a new checking account
/// await invoke('add_account', {
///     name: 'Wells Fargo Checking',
///     accountType: 'checking'
/// });
///
/// // Handle validation errors
/// try {
///     await invoke('add_account', { name: '', accountType: 'invalid' });
/// } catch (error) {
///     console.error('Failed to create account:', error);
/// }
/// ```
#[tauri::command]
async fn add_account(
    db: tauri::State<'_, SqlitePool>,
    name: String,
    account_type: String,
) -> Result<(), String> {
    database::add_account(&*db, name, account_type)
        .await
        .map_err(|e| e.to_string())
}

/// Retrieves all transactions for a specific financial account.
///
/// Returns transaction records ordered by date (most recent first) as JSON-serializable
/// values for frontend display. Each transaction includes amount, type, description,
/// date, and optional balance information.
///
/// # Arguments
/// * `account_id` - Database ID of the account to query transactions for
/// * `db` - SQLite connection pool managed by Tauri state
///
/// # Returns
/// * `Ok(Vec<serde_json::Value>)` - Array of transaction objects ordered by date
/// * `Err(String)` - Database error message for frontend display
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Account ID does not exist (invalid foreign key reference)
/// - Query execution fails (corrupted database, schema mismatch)
/// - JSON serialization fails (malformed data in database)
///
/// # Examples
/// ```javascript
/// // Load transactions for account detail view
/// const transactions = await invoke('get_transactions_by_account', {
///     accountId: 1
/// });
///
/// transactions.forEach(tx => {
///     console.log(`${tx.date}: ${tx.description} - $${tx.amount_cents / 100}`);
/// });
/// ```
#[tauri::command]
async fn get_transactions(
    account_id: i64,
    db: tauri::State<'_, SqlitePool>,
) -> Result<Vec<serde_json::Value>, String> {
    database::get_transactions(&*db, account_id)
        .await
        .map_err(|e| e.to_string())
}

/// Creates a new financial transaction record for the specified account.
///
/// Validates input parameters and inserts a transaction with the current timestamp.
/// Uses integer cents to avoid floating-point precision issues common in financial
/// applications. Transaction types determine how amounts affect account balances.
///
/// # Arguments
/// * `account_id` - Database ID of the account this transaction belongs to
/// * `amount_cents` - Transaction amount in cents (always positive, e.g., 2550 for $25.50)
/// * `transaction_type` - Either "debit" (reduces balance) or "credit" (increases balance)
/// * `description` - Human-readable transaction description from bank or user input
/// * `transaction_date` - Transaction date in ISO 8601 format (YYYY-MM-DD)
/// * `db` - SQLite connection pool managed by Tauri state
///
/// # Returns
/// * `Ok(())` - Transaction created successfully
/// * `Err(String)` - Validation or database error message for frontend display
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Account ID does not exist (invalid foreign key reference)
/// - Invalid transaction type provided (must be "debit" or "credit")
/// - Date format is invalid (must be YYYY-MM-DD ISO 8601)
/// - Database insertion fails (disk space, permissions, corruption)
///
/// # Examples
/// ```javascript
/// // Add a grocery store purchase
/// await invoke('add_transaction', {
///     accountId: 1,
///     amountCents: 4275,  // $42.75
///     transactionType: 'debit',
///     description: 'Whole Foods Market',
///     transactionDate: '2025-08-15'
/// });
///
/// // Add a paycheck deposit
/// await invoke('add_transaction', {
///     accountId: 1,
///     amountCents: 250000,  // $2,500.00
///     transactionType: 'credit',
///     description: 'Salary Deposit',
///     transactionDate: '2025-08-15'
/// });
/// ```
#[tauri::command]
async fn add_transaction(
    account_id: i64,
    amount_cents: i64,
    transaction_type: String,
    description: String,
    transaction_date: String,
    db: tauri::State<'_, SqlitePool>,
) -> Result<(), String> {
    database::add_transaction(
        &*db,
        account_id,
        amount_cents,
        transaction_type,
        description,
        transaction_date,
    )
    .await
    .map_err(|e| e.to_string())
}
