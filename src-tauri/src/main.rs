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
        .invoke_handler(tauri::generate_handler![get_accounts])
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
