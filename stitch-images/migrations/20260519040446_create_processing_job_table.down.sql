-- Add down migration script here
DROP TABLE IF EXISTS image_processing_jobs CASCADE;
DROP FUNCTION IF EXISTS update_updated_at_column;