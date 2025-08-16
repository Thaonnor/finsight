//! Database migration system for the finsight personal finance application.
//!
//! Provides automatic schema evolution through versioned migrations that run
//! on application startup. Each migration is tracked in the database to ensure
//! they only run once and can be safely applied to existing data.
//!
//! # Migration System Design
//!
//! - **Registry Pattern**: All migrations are registered in `run_migrations()`
//! - **Automatic Execution**: Migrations run during database initialization
//! - **Tracking**: Applied migrations are recorded in the `migrations` table
//! - **One-Time Execution**: Each migration runs only once per database
//! - **Sequential Naming**: Migrations use numbered prefixes (001_, 002_, etc.)
//!
//! # Adding New Migrations
//!
//! 1. Create a new migration function: `migration_XXX_description`
//! 2. Add it to the registry in `run_migrations()`
//! 3. Migrations will automatically run on next app startup
//!
//! # Example Migration
//!
//! ```rust
//! async fn migration_002_add_user_field(pool: &SqlitePool) -> Result<(), sqlx::Error> {
//!     sqlx::query("ALTER TABLE accounts ADD COLUMN user_id INTEGER")
//!         .execute(pool)
//!         .await?;
//!     Ok(())
//! }
//! ```
//!
//! # Safety
//!
//! - Migrations should be backward-compatible when possible
//! - Use `DEFAULT` values for new required columns
//! - Test migrations against real data during development

use sqlx::{Row, SqlitePool};

/// Executes all pending database migrations in sequential order.
///
/// Checks the migrations table to determine which migrations have already been
/// applied, then runs any missing migrations from the registry. Each migration
/// is executed exactly once and recorded in the migrations table to prevent
/// duplicate execution on future application starts.
///
/// # Migration Registry
///
/// All available migrations are defined in the `migration_registry` vector with
/// their names and corresponding functions. New migrations should be added to
/// this registry to be automatically discovered and executed.
///
/// # Arguments
/// * `pool` - SQLite connection pool for executing migrations and tracking
///
/// # Returns
/// * `Ok(())` - All pending migrations completed successfully
/// * `Err(sqlx::Error)` - Migration execution or tracking failure
///
/// # Errors
/// Fails if:
/// - Cannot query existing migrations from database
/// - Migration function execution fails (SQL errors, schema conflicts)
/// - Cannot record migration completion in migrations table
/// - Database connection issues during migration process
///
/// # Examples
/// ```no_run
/// // Called automatically during database initialization
/// let pool = init_db().await?;
/// // Migrations have already been applied
/// ```
///
/// # Adding Migrations
/// ```rust
/// let migration_registry = vec![
///     ("001_add_archived_column", migration_001_add_archived_column),
///     ("002_new_migration", migration_002_new_migration), // <- Add here
/// ];
/// ```
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let applied = get_applied_migrations(pool).await?;

    let migration_registry = vec![("001_add_archived_column", migration_001_add_archived_column)];

    for (name, migration_fn) in migration_registry {
        if !applied.contains(&name.to_string()) {
            migration_fn(pool).await?;
            record_migration(pool, name).await?;
        }
    }

    Ok(())
}

/// Retrieves the names of all migrations that have been applied to the database.
///
/// Queries the migrations table and returns a list of migration names that have
/// already been executed. This list is used by the migration runner to determine
/// which migrations still need to be applied to bring the database schema up to date.
///
/// # Arguments
/// * `pool` - SQLite connection pool for querying the migrations table
///
/// # Returns
/// * `Ok(Vec<String>)` - List of migration names that have been applied
/// * `Err(sqlx::Error)` - Database query or data extraction failure
///
/// # Errors
/// Fails if:
/// - Cannot connect to database or query migrations table
/// - Migrations table doesn't exist (should be created by `create_tables()`)
/// - Row data extraction fails due to schema changes
/// - Database file corruption or permission issues
///
/// # Examples
/// ```no_run
/// let applied = get_applied_migrations(&pool).await?;
/// println!("Applied migrations: {:?}", applied);
/// // Output: ["001_add_archived_column", "002_add_user_field"]
/// ```
async fn get_applied_migrations(pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query("SELECT migration_name FROM migrations")
        .fetch_all(pool)
        .await?;

    let result: Vec<String> = rows
        .into_iter()
        .map(|row| row.get::<String, _>("migration_name"))
        .collect();

    Ok(result)
}

/// Records a successfully applied migration in the migrations table.
///
/// Inserts the migration name into the migrations table with an automatic timestamp
/// to track when it was applied. This prevents the migration from being executed
/// again on future application starts and provides an audit trail of schema changes.
///
/// # Arguments
/// * `pool` - SQLite connection pool for inserting the migration record
/// * `migration_name` - Name of the migration that was successfully applied
///
/// # Returns
/// * `Ok(())` - Migration recorded successfully in the database
/// * `Err(sqlx::Error)` - Database insertion or connection failure
///
/// # Errors
/// Fails if:
/// - Cannot connect to database or access migrations table
/// - Migration name violates database constraints (duplicate entries)
/// - Database insertion fails due to permissions or disk space
/// - Connection pool exhaustion or database file locks
///
/// # Examples
/// ```no_run
/// // Called automatically after successful migration execution
/// migration_001_add_archived_column(&pool).await?;
/// record_migration(&pool, "001_add_archived_column").await?;
/// ```
///
/// # Database Record
/// Creates a record with migration name and automatic timestamp:
/// ```sql
/// INSERT INTO migrations (migration_name) VALUES ('001_add_archived_column')
/// -- Applied_at timestamp set automatically by database
/// ```
async fn record_migration(pool: &SqlitePool, migration_name: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO migrations (migration_name) VALUES (?)")
        .bind(migration_name)
        .execute(pool)
        .await?;

    Ok(())
}

/// Adds an archived column to the accounts table for soft deletion functionality.
///
/// This migration introduces account archiving as an alternative to hard deletion,
/// preserving historical financial data while allowing accounts to be hidden from
/// active use. All existing accounts are automatically set to not archived (FALSE)
/// when the column is added.
///
/// # Schema Changes
/// - Adds `archived BOOLEAN NOT NULL DEFAULT FALSE` column to accounts table
/// - Existing accounts receive archived = FALSE automatically
/// - New accounts default to archived = FALSE unless explicitly set
///
/// # Arguments
/// * `pool` - SQLite connection pool for executing the schema change
///
/// # Returns
/// * `Ok(())` - Column added successfully to accounts table
/// * `Err(sqlx::Error)` - Schema modification or database access failure
///
/// # Errors
/// Fails if:
/// - Accounts table doesn't exist (should be created by `create_tables()`)
/// - Column already exists (migration previously applied)
/// - Database schema modification permissions denied
/// - Insufficient disk space for table restructuring
///
/// # Impact on Existing Data
/// - All current account records get archived = FALSE
/// - No data loss or corruption - purely additive change
/// - Maintains backward compatibility with existing account queries
///
/// # Usage After Migration
/// ```sql
/// -- Hide account instead of deleting
/// UPDATE accounts SET archived = TRUE WHERE id = 123;
/// 
/// -- Query only active accounts
/// SELECT * FROM accounts WHERE archived = FALSE;
/// ```
async fn migration_001_add_archived_column(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // TODO: Add archived column
    sqlx::query("ALTER TABLE accounts ADD COLUMN archived BOOLEAN NOT NULL DEFAULT FALSE")
        .execute(pool)
        .await?;

    Ok(())
}
