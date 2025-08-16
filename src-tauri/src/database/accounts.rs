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
