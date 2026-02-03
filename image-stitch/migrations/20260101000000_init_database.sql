-- Initialize database extensions and settings for image-stitch
-- This migration runs first and sets up the database environment

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enable pgcrypto for encryption functions
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create image_stitch schema for this service
CREATE SCHEMA IF NOT EXISTS image_stitch;