//! Categories database operations for transaction categorization.
//!
//! Provides CRUD operations for hierarchical categories with parent-child relationships.
//! Categories are required for all transactions and support nested organization for
//! detailed expense tracking and analysis.

use sqlx::{Row, SqlitePool};

/// Retrieves all categories from the database.
///
/// Queries the categories table and returns all records as JSON-serializable objects
/// for frontend consumption. Results include category ID, name, and parent relationship
/// but exclude internal timestamps to keep the API clean.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing the query
///
/// # Returns
/// * `Ok(Vec<serde_json::Value>)` - Array of category objects with id, name, and parent_id
/// * `Err(sqlx::Error)` - Database query or serialization failure
pub async fn get_all_categories(pool: &SqlitePool) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let categories = sqlx::query("SELECT id, name, parent_id FROM categories")
        .fetch_all(pool)
        .await?;

    let result: Vec<serde_json::Value> = categories
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<i64, _>("id"),
                "name": row.get::<String, _>("name"),
                "parent_id": row.get::<Option<i64>, _>("parent_id")
            })
        })
        .collect();

    Ok(result)
}

/// Creates a new category in the database.
///
/// Inserts a new category record with the provided name and optional parent relationship.
/// The creation timestamp is automatically set by the database. Category names should be
/// descriptive for meaningful transaction organization.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing the insertion
/// * `name` - Human-readable category name (e.g., "Groceries", "Utilities")
/// * `parent_id` - Optional parent category ID for hierarchical organization
///
/// # Returns
/// * `Ok(())` - Category created successfully with auto-generated ID
/// * `Err(sqlx::Error)` - Database insertion or validation failure
pub async fn add_category(
    pool: &SqlitePool,
    name: String,
    parent_id: Option<i64>,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO categories (name, parent_id) VALUES (?, ?)")
        .bind(name)
        .bind(parent_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Updates an existing category with new values.
///
/// Modifies all fields of the specified category with the provided data.
/// This replaces the entire category record to ensure consistency across all
/// category attributes. The category ID remains immutable as the record identifier.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing the update
/// * `category_id` - Database ID of the category to modify
/// * `name` - New human-readable category name
/// * `parent_id` - New parent category ID for hierarchical organization (or None for root level)
///
/// # Returns
/// * `Ok(())` - Category updated successfully
/// * `Err(sqlx::Error)` - Database update failure or category not found
pub async fn update_category(
    pool: &SqlitePool,
    category_id: i64,
    name: String,
    parent_id: Option<i64>,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE categories SET name = ?, parent_id = ? WHERE id = ?")
        .bind(name)
        .bind(parent_id)
        .bind(category_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_category(pool: &SqlitePool, category_id: i64) -> Result<(), sqlx::Error> {
    handle_orphaned_categories(pool, category_id).await?;
    handle_orphaned_transactions(pool, category_id).await?;

    sqlx::query("DELETE FROM categories WHERE id = ?")
        .bind(category_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Reassigns child categories when their parent is deleted.
///
/// Promotes all child categories up one level in the hierarchy by inheriting
/// the deleted category's parent_id. If the deleted category was a root-level
/// category (no parent), children become root-level categories with NULL parent_id.
/// This preserves the category hierarchy structure while preventing orphaned references.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing database operations
/// * `category_id` - Database ID of the category being deleted
///
/// # Returns
/// * `Ok(())` - Child categories reassigned successfully
/// * `Err(sqlx::Error)` - Database query or update failure
///
/// # Examples
/// ```no_run
/// // Before: Discretionary -> Electronics -> Computers
/// // Delete Electronics (ID 2)
/// handle_orphaned_categories(&pool, 2).await?;
/// // After: Discretionary -> Computers (Electronics children inherit Discretionary as parent)
/// ```
async fn handle_orphaned_categories(
    pool: &SqlitePool,
    category_id: i64,
) -> Result<(), sqlx::Error> {
    let children = sqlx::query("SELECT id FROM categories WHERE parent_id = ?")
        .bind(category_id)
        .fetch_all(pool)
        .await?;

    if children.is_empty() {
        return Ok(()); // No children, no orphaning
    }

    let parent_row = sqlx::query("SELECT parent_id FROM categories WHERE id = ?")
        .bind(category_id)
        .fetch_all(pool)
        .await?;

    let parent_id: Option<i64> = if !parent_row.is_empty() {
        parent_row[0].get("parent_id")
    } else {
        None
    };

    sqlx::query("UPDATE categories SET parent_id = ? WHERE parent_id = ?")
        .bind(parent_id)
        .bind(category_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Reassigns orphaned transactions when their category is deleted.
///
/// Moves all transactions from the deleted category to the "Uncategorized" 
/// system category to prevent foreign key violations and ensure transaction
/// data remains accessible. The "Uncategorized" category must exist in the
/// database for this operation to succeed.
///
/// # Arguments
/// * `pool` - SQLite connection pool reference for executing database operations
/// * `category_id` - Database ID of the category being deleted
///
/// # Returns
/// * `Ok(())` - Transactions reassigned successfully
/// * `Err(sqlx::Error)` - Database query failure or "Uncategorized" category not found
///
/// # Examples
/// ```no_run
/// // Move all transactions from deleted "Groceries" category to "Uncategorized"
/// handle_orphaned_transactions(&pool, 5).await?;
/// ```
async fn handle_orphaned_transactions(
    pool: &SqlitePool,
    category_id: i64,
) -> Result<(), sqlx::Error> {
    let uncategorized_row = sqlx::query("SELECT id FROM categories WHERE name = 'Uncategorized'")
        .fetch_one(pool)
        .await?;
    let uncategorized_id: i64 = uncategorized_row.get("id");

    sqlx::query("UPDATE transactions SET category_id = ? WHERE category_id = ?")
        .bind(uncategorized_id)
        .bind(category_id)
        .execute(pool)
        .await?;

    Ok(())
}
