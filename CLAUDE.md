# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**CRITICAL: DO NOT EDIT FILES**

-   Never use Edit, MultiEdit, Write, or NotebookEdit tools
-   Never create new files unless explicitly requested
-   Only READ files to answer questions
-   If providing code examples or suggestions, display them in terminal output only
-   Do not modify existing code - only analyze and discuss it
-   User will handle all file modifications manually

## Development Commands

**Frontend Development:**

-   `npm run dev` - Start Vite development server (port 5173)
-   `npm run build` - Build frontend for production
-   `npm run tauri:dev` - Start Tauri development mode with hot reload
-   `npm run tauri` - Access Tauri CLI commands

**Backend Development:**

-   `cargo test --manifest-path src-tauri/Cargo.toml` - Run Rust tests
-   `cargo doc --manifest-path src-tauri/Cargo.toml --no-deps --open` - Generate and open Rust documentation

## Architecture Overview

**Technology Stack:**

-   **Frontend**: Vue 3 with Composition API, Vuetify 3 (Material Design)
-   **Backend**: Rust with Tauri 2.0 for cross-platform desktop
-   **Database**: SQLite with SQLx for local persistence
-   **Routing**: Vue Router 4 with lazy loading

**Project Structure:**

-   `src/` - Vue frontend application
    -   `views/` - Page-level Vue components with routing
    -   `components/` - Reusable Vue components
    -   `utils/` - Frontend utilities (currency formatting, date handling)
-   `src-tauri/` - Rust backend with Tauri commands
    -   `src/database/` - Database layer with SQLx operations
    -   `src/main.rs` - Tauri command handlers and application entry

**Database Architecture:**

-   **Storage Pattern**: Signed integer cents (debit transactions are negative, credit transactions are positive)
-   **Schema**: accounts -> transactions (1:many), categories -> transactions (1:many)
-   **File Location**: `finsight.db` in current working directory
-   **Migration System**: Automated schema versioning in `database/migrations.rs`

**Frontend-Backend Communication:**

-   Tauri commands via `@tauri-apps/api/core` invoke()
-   All Rust functions exposed as async commands in main.rs
-   JSON serialization for complex data types
-   Error handling with Result<T, String> pattern

**Key Design Patterns:**

-   **Financial Precision**: All monetary values stored as integer cents to avoid floating-point errors
-   **Reactive Data**: Vue Composition API with reactive refs for real-time updates
-   **Component Architecture**: Vuetify components for consistent Material Design
-   **Database Operations**: Connection pooling with prepared statements for security

**Current Vuetify Migration Status:**

-   ✅ `AddTransactionModal.vue` - Fully converted to Vuetify components
-   ✅ `App.vue` - Using v-app, v-navigation-drawer, v-main layout
-   ✅ `Sidebar.vue` - Partially converted (v-toolbar, v-list structure in progress)
-   ⚠️ `AccountDetails.vue` - Modern Vue patterns but still uses custom CSS
-   🔄 Other views - Not yet converted to Vuetify

**Route Structure:**

-   `/` - Dashboard (financial overview)
-   `/accounts` - Account management
-   `/accounts/:id` - Individual account transaction details
-   `/categories` - Transaction category management
-   `/import` - Data import functionality
-   `/reports` - Financial reporting (TODO: implementation pending)

**Financial Data Model:**

-   **Accounts**: checking/savings with unique names and archive status
-   **Transactions**: linked to accounts with category classification
-   **Categories**: hierarchical system with "Uncategorized" as default
-   **Balance Calculation**: Computed from transaction history, not stored
