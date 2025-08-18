use sqlx::{Row, SqlitePool};

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

/// Updates an existing financial account with new values.
///
/// Modifies all fields of the specified account with the provided data,
/// including the archived status for soft deletion functionality. This
/// replaces the entire account record to ensure consistency across all
/// account attributes. The account ID remains immutable as the record identifier.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing the update
/// * `account_id` - Database ID of the account to modify
/// * `name` - New human-readable account name
/// * `account_type` - New account classification ("checking" or "savings")
/// * `archived` - New archived status (true hides account, false shows it)
///
/// # Returns
/// * `Ok(())` - Account updated successfully
/// * `Err(sqlx::Error)` - Database update failure or account not found
///
/// # Errors
/// Fails if:
/// - Database connection cannot be established (pool exhaustion, file locks)
/// - Account ID does not exist (no matching record to update)
/// - Account name violates constraints (empty string, potential duplicates)
/// - Account type is invalid or unsupported by application logic
/// - Database update fails (permissions, corruption, constraint violations)
/// - Parameter binding fails (invalid UTF-8 in strings)
///
/// # Examples
/// ```no_run
/// // Rename an account
/// update_account(
///     &pool,
///     1,
///     "Chase Premium Checking".to_string(),
///     "checking".to_string(),
///     false
/// ).await?;
///
/// // Archive an old account
/// update_account(
///     &pool,
///     5,
///     "Old Savings Account".to_string(),
///     "savings".to_string(),
///     true
/// ).await?;
/// ```
pub async fn update_account(
    pool: &SqlitePool,
    account_id: i64,
    name: String,
    account_type: String,
    archived: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE accounts SET name = ?, account_type = ?, archived = ? WHERE id = ?")
        .bind(name)
        .bind(account_type)
        .bind(archived)
        .bind(account_id)
        .execute(pool)
        .await?;

    Ok(())
}