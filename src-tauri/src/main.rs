// Prevents console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! finsight - Personal Finance Desktop Application
//!
//! A Tauri-based desktop app for transforming household financial data
//! into business-style financial statements and insights.

mod database;
use sqlx::SqlitePool;

/// Main entry point for the finsight Tauri application.
///
/// Initializes and runs the Tauri app with default configuration.
/// The app will display a webview containing the Vue.js frontend.
///
/// # Errors
///
/// This function will return an error if:
/// * Database initialization fails (file permissions, disk space, etc.)
/// * Tauri application fails to start (missing dependencies, display issues)
///
/// # Panics
///
/// Panics if Tauri context generation fails, which indicates a build configuration problem.
///
/// # Examples
///
/// This function is called automatically when the application starts:
/// ```no_run
/// // Called by the Rust runtime when app launches
/// main().await;
/// ```
#[tokio::main]
async fn main() {
    // Initialize the database
    let db_pool = database::init_db()
        .await
        .expect("Failed to initialize database");

    tauri::Builder::default()
        .manage(db_pool)
        .invoke_handler(tauri::generate_handler![
            get_accounts,
            add_account,
            get_transactions_by_account,
            add_transaction
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

/// Fetch all accounts from the database
///
/// # Arguments
///
/// * `db` - Database connection pool from Tauri's managed state
///
/// # Returns
///
/// A vector of account data as JSON-serializable values
#[tauri::command]
async fn get_accounts(db: tauri::State<'_, SqlitePool>) -> Result<Vec<serde_json::Value>, String> {
    database::get_all_accounts(&*db)
        .await
        .map_err(|e| e.to_string())
}

/// Add a new account to the database
///
/// #Arguments
///
/// * `db` - Database connection pool from Tauri's managed state
/// * `name` - The account name (e.g., Chase Checking)
/// * `account_type` - The account type ("checking" or "savings")
///
/// # Returns
///
/// Empty result on success, or error string on failure
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

/// Get all transactions for a specific account
///
/// Retrieves all transactions associated with a given account ID, returning them
/// as JSON objects with transaction details including amount, type, description, and date.
///
/// # Arguments
///
/// * `account_id` - The ID of the account to retrieve transactions for
/// * `db` - Tauri-managed database connection pool state
///
/// # Returns
///
/// * `Ok(Vec<serde_json::Value>)` - List of transactions as JSON objects
/// * `Err(String)` - Database error converted to string for frontend consumption
///
/// # Examples
///
/// ```javascript
/// // Get transactions for account with ID 1
/// const transactions = await invoke('get_transactions_by_account', { accountId: 1 });
/// ```
#[tauri::command]
async fn get_transactions_by_account(
    account_id: i64,
    db: tauri::State<'_, SqlitePool>,
) -> Result<Vec<serde_json::Value>, String> {
    database::get_transactions_by_account(&*db, account_id)
        .await
        .map_err(|e| e.to_string())
}

/// Add a new transaction to an account
///
/// Creates a new transaction record for the specified account with the provided
/// details. All amounts should be positive integers representing cents.
///
/// # Arguments
///
/// * `account_id` - The ID of the account this transaction belongs to
/// * `amount_cents` - Transaction amount in cents (always positive, e.g., 2550 for $25.50)
/// * `transaction_type` - Either "debit" or "credit"
/// * `description` - Description of the transaction
/// * `transaction_date` - Transaction date in ISO 8601 format (YYYY-MM-DD)
/// * `db` - Tauri-managed database connection pool state
///
/// # Returns
///
/// * `Ok(())` - Transaction added successfully
/// * `Err(String)` - Database error converted to string for frontend consumption
///
/// # Examples
///
/// ```javascript
/// // Add a $25.50 debit transaction
/// await invoke('add_transaction', {
///     accountId: 1,
///     amountCents: 2550,
///     transactionType: 'debit',
///     description: 'Coffee shop',
///     transactionDate: '2025-08-13'
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
