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
/// - `update_account` - Updates existing account details and archived status
/// - `get_transactions` - Fetches transactions for a specific account
/// - `add_transaction` - Creates a new transaction record
/// - `delete_transaction` - Permanently removes a transaction record
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
            get_account,
            add_account,
            update_account,
            get_balance,
            get_transactions,
            add_transaction,
            delete_transaction,
            update_transaction,
            get_categories,
            add_category,
            update_category,
            delete_category
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

#[tauri::command]
async fn get_account(
    db: tauri::State<'_, SqlitePool>,
    account_id: i64,
) -> Result<serde_json::Value, String> {
    database::get_account(&*db, account_id)
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

/// Updates an existing financial account with new values.
///
/// Modifies all fields of the specified account including name, type, and archived
/// status. This allows for complete account management including soft deletion
/// through the archived flag. All parameters are required to ensure data consistency.
///
/// # Arguments
/// * `db` - SQLite connection pool managed by Tauri state
/// * `account_id` - Database ID of the account to modify
/// * `name` - New human-readable account name
/// * `account_type` - New account classification ("checking" or "savings")
/// * `archived` - New archived status (true hides account, false shows it)
///
/// # Returns
/// * `Ok(())` - Account updated successfully
/// * `Err(String)` - Validation or database error message for frontend display
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Account ID does not exist (no matching record to update)
/// - Account name violates constraints (empty string, potential duplicates)
/// - Invalid account type provided (must be "checking" or "savings")
/// - Database update fails (permissions, corruption, constraint violations)
///
/// # Examples
/// ```javascript
/// // Rename an account
/// await invoke('update_account', {
///     accountId: 1,
///     name: 'Chase Premium Checking',
///     accountType: 'checking',
///     archived: false
/// });
///
/// // Archive an old account
/// await invoke('update_account', {
///     accountId: 5,
///     name: 'Old Savings Account',
///     accountType: 'savings',
///     archived: true
/// });
/// ```
#[tauri::command]
async fn update_account(
    db: tauri::State<'_, SqlitePool>,
    account_id: i64,
    name: String,
    account_type: String,
    archived: bool,
) -> Result<(), String> {
    database::update_account(&*db, account_id, name, account_type, archived)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_balance(db: tauri::State<'_, SqlitePool>, account_id: i64) -> Result<i64, String> {
    database::get_balance(&*db, account_id)
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
/// const transactions = await invoke('get_transactions', {
///     accountId: 1
/// });
///
/// transactions.forEach(tx => {
///     console.log(`${tx.date}: ${tx.description} - $${tx.amount_cents / 100}`);
/// });
/// ```
#[tauri::command]
async fn get_transactions(
    db: tauri::State<'_, SqlitePool>,
    account_id: i64,
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
    db: tauri::State<'_, SqlitePool>,
    account_id: i64,
    amount_cents: i64,
    transaction_type: String,
    description: String,
    transaction_date: String,
    category_id: i64,
) -> Result<(), String> {
    database::add_transaction(
        &*db,
        account_id,
        amount_cents,
        transaction_type,
        description,
        transaction_date,
        category_id,
    )
    .await
    .map_err(|e| e.to_string())
}

/// Permanently removes a transaction record from the database.
///
/// Deletes the transaction with the specified ID from the database. This operation
/// cannot be undone and will completely remove the transaction from financial records.
/// Use with caution as this affects historical data and account balance calculations.
///
/// # Arguments
/// * `db` - SQLite connection pool managed by Tauri state
/// * `transaction_id` - Database ID of the transaction to remove
///
/// # Returns
/// * `Ok(())` - Transaction deleted successfully
/// * `Err(String)` - Database error message for frontend display
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Transaction ID does not exist (no matching record to delete)
/// - Database deletion fails (permissions, corruption, foreign key constraints)
/// - Connection pool is exhausted or disconnected
///
/// # Examples
/// ```javascript
/// // Remove an incorrect transaction entry
/// await invoke('delete_transaction', {
///     transactionId: 123
/// });
///
/// // Handle deletion errors
/// try {
///     await invoke('delete_transaction', { transactionId: 999 });
/// } catch (error) {
///     console.error('Failed to delete transaction:', error);
/// }
/// ```
#[tauri::command]
async fn delete_transaction(
    db: tauri::State<'_, SqlitePool>,
    transaction_id: i64,
) -> Result<(), String> {
    database::delete_transaction(&*db, transaction_id)
        .await
        .map_err(|e| e.to_string())
}

/// Updates an existing transaction record with new values.
///
/// Modifies all fields of the specified transaction with the provided data.
/// This replaces the entire transaction record, ensuring consistency across
/// all transaction attributes. Useful for correcting transaction details or
/// moving transactions between accounts.
///
/// # Arguments
/// * `db` - SQLite connection pool managed by Tauri state
/// * `transaction_id` - Database ID of the transaction to modify
/// * `account_id` - New account ID this transaction belongs to
/// * `amount_cents` - New transaction amount in cents (always positive)
/// * `transaction_type` - New transaction type ("debit" or "credit")
/// * `description` - New human-readable transaction description
/// * `transaction_date` - New transaction date in ISO 8601 format (YYYY-MM-DD)
///
/// # Returns
/// * `Ok(())` - Transaction updated successfully
/// * `Err(String)` - Database error message for frontend display
#[tauri::command]
async fn update_transaction(
    db: tauri::State<'_, SqlitePool>,
    transaction_id: i64,
    account_id: i64,
    amount_cents: i64,
    transaction_type: String,
    description: String,
    transaction_date: String,
    category_id: i64,
) -> Result<(), String> {
    database::update_transaction(
        &*db,
        transaction_id,
        account_id,
        amount_cents,
        transaction_type,
        description,
        transaction_date,
        category_id,
    )
    .await
    .map_err(|e| e.to_string())
}

/// Retrieves all categories from the database for transaction categorization.
///
/// Returns category records as JSON-serializable values including hierarchical
/// relationships. Categories include system categories like "Uncategorized" and
/// user-defined categories for organizing transactions.
///
/// # Arguments
/// * `db` - SQLite connection pool managed by Tauri state
///
/// # Returns
/// * `Ok(Vec<serde_json::Value>)` - Array of category objects with id, name, and parent_id
/// * `Err(String)` - Database error message for frontend display
#[tauri::command]
async fn get_categories(
    db: tauri::State<'_, SqlitePool>,
) -> Result<Vec<serde_json::Value>, String> {
    database::get_all_categories(&*db)
        .await
        .map_err(|e| e.to_string())
}

/// Creates a new category for transaction organization.
///
/// Inserts a category record with optional parent relationship for hierarchical
/// organization. Category names must be unique across the entire system to prevent
/// confusion in transaction categorization.
///
/// # Arguments
/// * `db` - SQLite connection pool managed by Tauri state
/// * `name` - Unique category name (e.g., "Groceries", "Utilities")
/// * `parent_id` - Optional parent category ID for hierarchical organization
///
/// # Returns
/// * `Ok(())` - Category created successfully
/// * `Err(String)` - Database error message for frontend display
#[tauri::command]
async fn add_category(
    db: tauri::State<'_, SqlitePool>,
    name: String,
    parent_id: Option<i64>,
) -> Result<(), String> {
    database::add_category(&*db, name, parent_id)
        .await
        .map_err(|e| e.to_string())
}

/// Updates an existing category with new values.
///
/// Modifies category name and parent relationship. Useful for reorganizing
/// category hierarchies or correcting category names. All transactions
/// using this category remain properly linked.
///
/// # Arguments
/// * `db` - SQLite connection pool managed by Tauri state
/// * `category_id` - Database ID of the category to modify
/// * `name` - New unique category name
/// * `parent_id` - New parent category ID or None for root level
///
/// # Returns
/// * `Ok(())` - Category updated successfully
/// * `Err(String)` - Database error message for frontend display
#[tauri::command]
async fn update_category(
    db: tauri::State<'_, SqlitePool>,
    category_id: i64,
    name: String,
    parent_id: Option<i64>,
) -> Result<(), String> {
    database::update_category(&*db, category_id, name, parent_id)
        .await
        .map_err(|e| e.to_string())
}

/// Removes a category with automatic cleanup of dependent data.
///
/// Deletes the category and handles orphaned data by moving child categories
/// up one level in the hierarchy and reassigning all transactions to the
/// "Uncategorized" system category. Cannot delete the "Uncategorized" category itself.
///
/// # Arguments
/// * `db` - SQLite connection pool managed by Tauri state
/// * `category_id` - Database ID of the category to remove
///
/// # Returns
/// * `Ok(())` - Category deleted successfully with cleanup completed
/// * `Err(String)` - Database error message for frontend display
#[tauri::command]
async fn delete_category(db: tauri::State<'_, SqlitePool>, category_id: i64) -> Result<(), String> {
    database::delete_category(&*db, category_id)
        .await
        .map_err(|e| e.to_string())
}
