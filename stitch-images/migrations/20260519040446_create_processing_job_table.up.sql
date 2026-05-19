-- Add up migration script here
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;

DO $$ BEGIN
    CREATE TYPE job_status AS ENUM ('created', 'submission_complete', 'processing', 'completed', 'failed');
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

CREATE TABLE IF NOT EXISTS image_processing_jobs (
    -- id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    id UUID PRIMARY KEY DEFAULT public.uuid_generate_v4(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    status job_status NOT NULL DEFAULT 'created',
    image_count INTEGER NOT NULL,
    submitted_count INTEGER NOT NULL DEFAULT 0,
    error_message TEXT DEFAULT NULL
);

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create index on created_at for efficient ordering
CREATE INDEX IF NOT EXISTS idx_image_processing_jobs_created_at ON image_processing_jobs (created_at DESC);

-- Create index on status for efficient filtering
CREATE INDEX IF NOT EXISTS idx_image_processing_jobs_status ON image_processing_jobs (status);

-- Trigger to call the function before any UPDATE
CREATE TRIGGER update_image_processing_jobs_updated_at
    BEFORE UPDATE ON image_processing_jobs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
