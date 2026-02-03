-- Create image_processing_jobs table for tracking async image processing
CREATE TABLE IF NOT EXISTS image_processing_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    image_count INTEGER NOT NULL,
    result_path TEXT,
    error_message TEXT
);

-- Create index on created_at for efficient ordering
CREATE INDEX IF NOT EXISTS idx_image_processing_jobs_created_at ON image_processing_jobs (created_at DESC);

-- Create index on status for efficient filtering
CREATE INDEX IF NOT EXISTS idx_image_processing_jobs_status ON image_processing_jobs (status);

-- Trigger to call the function before any UPDATE
CREATE TRIGGER update_image_processing_jobs_updated_at
    BEFORE UPDATE ON image_processing_jobs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
