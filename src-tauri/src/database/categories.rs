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
    async fn test_add_category() {
        let pool = setup_test_db().await;

        add_category(&pool, "Groceries".to_string(), None)
            .await
            .unwrap();

        let categories = get_all_categories(&pool).await.unwrap();
        assert_eq!(categories.len(), 2); // 'Uncategorized', 'Groceries'

        let groceries = categories
            .iter()
            .find(|c| c["name"] == "Groceries")
            .unwrap();
        assert_eq!(groceries["name"], "Groceries");
        assert_eq!(groceries["parent_id"], serde_json::Value::Null);
    }

    #[tokio::test]
    async fn test_update_category() {
        let pool = setup_test_db().await;

        add_category(&pool, "Original Name".to_string(), None)
            .await
            .unwrap();
        update_category(&pool, 2, "Updated Name".to_string(), Some(1))
            .await
            .unwrap();

        let categories = get_all_categories(&pool).await.unwrap();
        let updated_category = categories.iter().find(|c| c["id"] == 2).unwrap();

        assert_eq!(updated_category["name"], "Updated Name");
        assert_eq!(updated_category["parent_id"], 1);
    }

    #[tokio::test]
    async fn test_delete_category() {
        let pool = setup_test_db().await;

        add_category(&pool, "Groceries".to_string(), None).await.unwrap();

        delete_category(&pool, 2).await.unwrap();

        let categories = get_all_categories(&pool).await.unwrap();

        assert_eq!(categories.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_category_with_children() {
        let pool = setup_test_db().await;

        // Create: Uncategorized (1) -> Food (2) -> Groceries (3)
        add_category(&pool, "Food".to_string(), None).await.unwrap(); // ID 2
        add_category(&pool, "Groceries".to_string(), Some(2)).await.unwrap(); // ID 3, parent is Food

        // Delete Food (2) - Groceries should become root-level
        delete_category(&pool, 2).await.unwrap();

        let categories = get_all_categories(&pool).await.unwrap();
        let groceries = categories.iter().find(|c| c["name"] == "Groceries").unwrap();

        assert_eq!(groceries["parent_id"], serde_json::Value::Null); // Should be promoted to root
    }

    #[tokio::test]
    async fn test_delete_middle_category_with_children() {
        let pool = setup_test_db().await;

        // Create: Uncategorized (1) -> Food (2) -> Groceries (3) -> Organic (4)
        add_category(&pool, "Food".to_string(), None).await.unwrap(); // ID 2
        add_category(&pool, "Groceries".to_string(), Some(2)).await.unwrap(); // ID 3, parent is Food
        add_category(&pool, "Organic".to_string(), Some(3)).await.unwrap(); // ID 4, parent is Groceries

        // Delete Groceries(3) - Organic should inherit Food (2) as parent
        delete_category(&pool, 3).await.unwrap();

        let categories = get_all_categories(&pool).await.unwrap();
        let organic = categories.iter().find(|c| c["name"] == "Organic").unwrap();

        assert_eq!(organic["parent_id"], 2); // Should inherit Food as parent
    }

    #[tokio::test]
    async fn test_delete_category_with_transactions() {
        let pool = setup_test_db().await;

        // Create account and category for the transaction
        crate::database::add_account(&pool, "Test Account".to_string(), "checking".to_string()).await.unwrap();
        add_category(&pool, "Food".to_string(), None).await.unwrap();

        // Create transaction in Food category
        crate::database::add_transaction(&pool, 1, -1000, "debit".to_string(), "groceries".to_string(), "2024-01-01".to_string(), 2).await.unwrap();

        // Delete Food category - transaction should move to Uncategorized (ID 1)
        delete_category(&pool, 2).await.unwrap();

        // Verify transaction moved to Uncategorized
        let transactions = crate::database::get_transactions(&pool, 1).await.unwrap();
        assert_eq!(transactions[0]["category_id"], 1);
    }
}
