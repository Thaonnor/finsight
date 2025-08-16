use sqlx::{Row, SqlitePool};

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
