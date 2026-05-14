//! # Database Initialization Library
//!
//! A reusable library for initializing PostgreSQL database connections with sqlx.
//! Provides configuration, connection pooling, schema management, and migrations.
//!
//! ## Features
//!
//! - PostgreSQL connection pool setup with configurable options
//! - Schema selection and verification
//! - Embedded migration support
//! - Optional schema auto-creation
//! - Connection health checks
//!
//! ## Example
//!
//! ```ignore
//! use db_init::{DbConfig, init_pool, run_migrations};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = DbConfig {
//!         host: "localhost".to_string(),
//!         port: 5432,
//!         database: "mydb".to_string(),
//!         username: "user".to_string(),
//!         password: "pass".to_string(),
//!         schema: "public".to_string(),
//!         max_connections: 5,
//!         acquire_timeout_secs: 3,
//!         ..Default::default()
//!     };
//!
//!     let pool = init_pool(&config).await?;
//!     // Run migrations from caller's embedded migrator
//!     // run_migrations(&pool, &my_migrator).await?;
//!     
//! #   Ok(())
//! # }
//! ```

#[cfg(feature = "postgres")]
pub mod postgres;

mod error;

pub use error::DbInitError;

#[cfg(feature = "postgres")]
pub use postgres::{init_pool, run_migrations, verify_schema, DbConfig, SchemaMode};
