// Prevents console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! finsight - Personal Finance Desktop Application
//!
//! A Tauri-based desktop app for transforming household financial data
//! into business-style financial statements and insights.

mod database;

/// Main entry point for the finsight Tauri application.
///
/// Initializes and runs the Tauri app with default configuration.
/// The app will display a webview containing the Vue.js frontend.
#[tokio::main]
async fn main() {
    // Initialize the database
    let _db_pool = database::init_db()
        .await
        .expect("Failed to initialize database");

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
