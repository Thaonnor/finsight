use sqlx::{Row, SqlitePool};

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

async fn record_migration(pool: &SqlitePool, migration_name: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO migrations (migration_name) VALUES (?)")
        .bind(migration_name)
        .execute(pool)
        .await?;

    Ok(())
}

async fn migration_001_add_archived_column(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // TODO: Add archived column
    sqlx::query("ALTER TABLE accounts ADD COLUMN archived BOOLEAN NOT NULL DEFAULT FALSE")
        .execute(pool)
        .await?;

    Ok(())
}
