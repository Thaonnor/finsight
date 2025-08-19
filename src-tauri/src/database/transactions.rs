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
/// let transactions = get_transactions(&pool, 1).await?;
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
    let transactions = sqlx::query("SELECT id, account_id, amount_cents, transaction_type, description, transaction_date, category_id FROM transactions WHERE account_id = ?").bind(account_id).fetch_all(pool).await?;

    let result: Vec<serde_json::Value> = transactions
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<i64, _>("id"),
                "account_id": row.get::<i64, _>("account_id"),
                "amount_cents": row.get::<i64, _>("amount_cents"),
                "transaction_type": row.get::<String, _>("transaction_type"),
                "description": row.get::<String, _>("description"),
                "transaction_date": row.get::<String, _>("transaction_date"),
                "category_id": row.get::<i64, _>("category_id")
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
    category_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO transactions (
        account_id,
        amount_cents,
        transaction_type,
        description,
        transaction_date,
        category_id) 
        VALUES (?, ?, ?, ?, ?, ?)
    "#,
    )
    .bind(account_id)
    .bind(amount_cents)
    .bind(transaction_type)
    .bind(description)
    .bind(transaction_date)
    .bind(category_id)
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

/// Updates an existing transaction record with new values.
///
/// Modifies all fields of the specified transaction with the provided data.
/// This replaces the entire transaction record, ensuring consistency across
/// all transaction attributes. The transaction ID cannot be changed through
/// this operation as it serves as the immutable record identifier.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing the update
/// * `transaction_id` - Database ID of the transaction to modify
/// * `account_id` - New account ID this transaction belongs to
/// * `amount_cents` - New transaction amount in cents (always positive, e.g., 2550 for $25.50)
/// * `transaction_type` - New transaction type ("debit" or "credit")
/// * `description` - New human-readable transaction description
/// * `transaction_date` - New transaction date in ISO 8601 format (YYYY-MM-DD)
///
/// # Returns
/// * `Ok(())` - Transaction updated successfully
/// * `Err(sqlx::Error)` - Database update failure or transaction not found
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Transaction ID does not exist (no matching record to update)
/// - New account ID does not exist (invalid foreign key reference)
/// - Transaction type is invalid (must be "debit" or "credit")
/// - Date format is malformed (must be valid ISO 8601 YYYY-MM-DD)
/// - Database update fails (permissions, corruption, constraint violations)
/// - Parameter binding fails (invalid UTF-8 in strings, integer overflow)
///
/// # Examples
/// ```no_run
/// // Correct a grocery store transaction amount
/// update_transaction(
///     &pool,
///     123,
///     1,
///     5275,  // Updated to $52.75
///     "debit".to_string(),
///     "Whole Foods Market - Corrected".to_string(),
///     "2025-08-15".to_string()
/// ).await?;
///
/// // Move transaction to different account
/// update_transaction(
///     &pool,
///     456,
///     2,      // New account_id
///     250000,
///     "credit".to_string(),
///     "Payroll Deposit".to_string(),
///     "2025-08-15".to_string()
/// ).await?;
/// ```
pub async fn update_transaction(
    pool: &SqlitePool,
    transaction_id: i64,
    account_id: i64,
    amount_cents: i64,
    transaction_type: String,
    description: String,
    transaction_date: String,
    category_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE transactions SET 
            account_id = ?,
            amount_cents = ?,
            transaction_type = ?,
            description = ?,
            transaction_date = ?,
            category_id = ?
            WHERE id = ?
            "#,
    )
    .bind(account_id)
    .bind(amount_cents)
    .bind(transaction_type)
    .bind(description)
    .bind(transaction_date)
    .bind(category_id)
    .bind(transaction_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        crate::database::create_tables(&pool).await.unwrap();
        crate::database::migrations::run_migrations(&pool)
            .await
            .unwrap();
        crate::database::seed_system_data(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_add_transaction() {
        let pool = setup_test_db().await;

        crate::database::add_account(&pool, "Test Account".to_string(), "checking".to_string())
            .await
            .unwrap();
        add_transaction(
            &pool,
            1,
            1000,
            "debit".to_string(),
            "Groceries".to_string(),
            "2025-01-01".to_string(),
            1,
        )
        .await
        .unwrap();

        let transactions = get_transactions(&pool, 1).await.unwrap();
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0]["amount_cents"], 1000);
        assert_eq!(transactions[0]["description"], "Groceries");
        assert_eq!(transactions[0]["category_id"], 1);
    }

    #[tokio::test]
    async fn test_update_transaction() {
        let pool = setup_test_db().await;

        crate::database::add_account(&pool, "Test Account".to_string(), "checking".to_string())
            .await
            .unwrap();

        // Add initial transaction
        add_transaction(
            &pool,
            1,
            1000,
            "debit".to_string(),
            "Original Description".to_string(),
            "2025-01-01".to_string(),
            1,
        )
        .await
        .unwrap();

        // Update transaction
        update_transaction(
            &pool,
            1,
            1,
            2000,
            "credit".to_string(),
            "Updated Description".to_string(),
            "2025-01-02".to_string(),
            1,
        )
        .await
        .unwrap();

        // Verify changes
        let transactions = get_transactions(&pool, 1).await.unwrap();
        assert_eq!(transactions[0]["amount_cents"], 2000);
        assert_eq!(transactions[0]["transaction_type"], "credit");
        assert_eq!(transactions[0]["description"], "Updated Description");
        assert_eq!(transactions[0]["transaction_date"], "2025-01-02");
    }

    #[tokio::test]
    async fn test_delete_transaction() {
        let pool = setup_test_db().await;

        crate::database::add_account(&pool, "Test Account".to_string(), "checking".to_string())
            .await
            .unwrap();

        add_transaction(
            &pool,
            1,
            1000,
            "debit".to_string(),
            "Groceries".to_string(),
            "2025-01-01".to_string(),
            1,
        )
        .await
        .unwrap();

        delete_transaction(&pool, 1).await.unwrap();
        
        let transactions = get_transactions(&pool, 1).await.unwrap();
        assert_eq!(transactions.len(), 0);
    }
}
