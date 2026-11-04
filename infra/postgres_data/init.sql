-- Bootstrap database extensions and service schemas on first Postgres init.
-- This script is only executed when the database directory is empty.

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";