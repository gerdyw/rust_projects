# Database Initialization Refactoring - Migration Notes

## Overview

This document describes the refactoring of duplicate database initialization code from `image-stitch` and `todo_web` into a shared `db_init` library crate.

## What Was Changed

### New Shared Crate: `crates/db_init`

A new library crate was created to provide reusable database initialization functionality:

**Public API:**
- `DbConfig`: Configuration struct for database connections
- `SchemaMode`: Enum controlling schema creation behavior (MustExist, CreateIfMissing)
- `DbInitError`: Unified error type for database initialization failures
- `init_pool()`: Initialize a PostgreSQL connection pool with schema support
- `run_migrations()`: Run embedded sqlx migrations
- `verify_schema()`: Verify connected schema matches expectations

**Features:**
- `postgres`: PostgreSQL backend support (required)
- `mysql`: Reserved for future use
- `sqlite`: Reserved for future use

### Migration Summary

| Aspect | Before | After |
|--------|--------|-------|
| Code duplication | ~130 lines duplicated | Single shared implementation |
| Pool configuration | Scattered across 2 files | Centralized in DbConfig |
| Schema handling | Different implementations | Unified with SchemaMode enum |
| Error types | 2 separate DbInitError types | Single shared error type |
| Migration strategy | Both used sqlx::migrate! | Caller passes migrator reference |

## Service-Specific Changes

### image-stitch

**Changes Made:**
- Added `db_init` dependency to `Cargo.toml`
- Converted `src/db/init.rs` to thin wrapper around shared crate
- Removed local `DbInitError` from `src/db/errors.rs`
- Re-exported `DbInitError` from shared crate for backward compatibility

**Behavioral Compatibility:**
- Schema mode: `MustExist` (schema must already exist)
- Pool settings: 5 max connections, 3 second acquire timeout
- Migrations: Still embedded with `sqlx::migrate!("./migrations")`
- Search path: Set via after_connect hook (handled by shared crate)

**Migration Pattern:**
```rust
// Before
let pool = init_pool(&settings.database).await?;
run_migrations(&pool).await?;

// After (internal implementation - public API unchanged)
let config = db_init::DbConfig {
    schema_mode: db_init::SchemaMode::MustExist,
    // ... other fields mapped from DatabaseSettings
};
let pool = db_init::init_pool(&config).await?;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
db_init::run_migrations(&pool, &MIGRATOR).await?;
```

### todo_web

**Changes Made:**
- Added `db_init` dependency to `Cargo.toml`
- Converted `src/db/mod.rs` to thin wrapper around shared crate
- Removed local `DbInitError` and helper functions
- Re-exported `DbInitError` from shared crate for backward compatibility

**Behavioral Compatibility:**
- Schema mode: `CreateIfMissing` (auto-creates schema if needed)
- Pool settings: 5 max connections, 3 second acquire timeout (now explicit)
- Migrations: Still embedded with `sqlx::migrate!()` (default path)
- Search path: Set via after_connect hook (handled by shared crate)
- Same logging output during initialization

**Migration Pattern:**
```rust
// Before
pub async fn init_db(db_settings: &DatabaseSettings) -> Pool<Postgres> {
    // Bootstrap connection for schema creation
    // Pool creation with after_connect hook
    // Manual verify_schema implementation
    // Run migrations inline
}

// After (internal implementation - public API unchanged)
pub async fn init_db(db_settings: &DatabaseSettings) -> Pool<Postgres> {
    let config = db_init::DbConfig {
        schema_mode: db_init::SchemaMode::CreateIfMissing,
        // ... other fields mapped from DatabaseSettings
    };
    let pool = db_init::init_pool(&config).await?;
    db_init::verify_schema(&pool, &db_settings.schema).await?;
    
    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();
    db_init::run_migrations(&pool, &MIGRATOR).await?;
    
    pool
}
```

## Breaking Changes

**None.** The refactoring was designed to maintain complete backward compatibility:

- Public APIs of `image-stitch` and `todo_web` remain unchanged
- Function signatures are identical
- Behavioral semantics are preserved
- Error types are re-exported with same names

## Benefits

1. **Reduced Duplication**: Eliminated ~130 lines of duplicate code
2. **Consistency**: Both services now use identical pool creation logic
3. **Maintainability**: Single source of truth for DB initialization
4. **Testability**: Shared code is unit tested
5. **Extensibility**: Easy to add MySQL/SQLite support via feature flags
6. **Documentation**: Comprehensive rustdoc with examples

## Configuration Mapping

### Field Name Differences

The shared crate uses standardized field names. Existing services map their fields:

| image-stitch | todo_web | db_init (shared) |
|--------------|----------|------------------|
| `name` | `db_name` | `database` |
| `user` | `username` | `username` |
| - | - | `max_connections` (explicit) |
| - | - | `acquire_timeout_secs` (explicit) |

## Testing

- **Unit tests**: Shared crate includes tests for identifier escaping and default configuration
- **Integration testing**: Both services build and pass existing tests
- **Clippy**: No new warnings introduced
- **Formatting**: All code formatted with `cargo fmt`

## Future Enhancements

Potential improvements for future consideration:

1. Add MySQL and SQLite backend support
2. Add connection health check / readiness probe helpers
3. Support for custom connection pool callbacks
4. Configuration validation helpers
5. Metrics/tracing integration
6. Connection retry logic
7. Environment-based configuration builders

## Security Considerations

- No new security vulnerabilities introduced
- SQL injection protection maintained via sqlx parameterized queries
- Schema identifier escaping properly handles quotes
- Connection credentials handled securely (not logged)

## Rollback Plan

If issues arise, revert commits in reverse order:

1. Revert formatting commit (ce8a610)
2. Revert todo_web migration (f3679cb)
3. Revert image-stitch migration (a115b9a)
4. Revert shared crate creation (6ba0307)

Each service will return to its independent implementation.

## Verification Checklist

- [x] Both services build successfully
- [x] Code properly formatted (`cargo fmt`)
- [x] Shared crate passes clippy without warnings
- [x] Unit tests pass
- [x] Public APIs unchanged
- [x] Error types properly exported
- [x] Documentation complete
- [x] Migration notes written

## Questions or Issues?

For questions about this refactoring or issues encountered:
1. Check this document for migration patterns
2. Review the shared crate README: `crates/db_init/README.md`
3. Examine the rustdoc: `cargo doc --open --features postgres`
