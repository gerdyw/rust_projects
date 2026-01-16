-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create example_entities table
CREATE TABLE IF NOT EXISTS example_entities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    name VARCHAR(255) NOT NULL,
    description TEXT
);

-- Create index on created_at for efficient ordering
CREATE INDEX IF NOT EXISTS idx_example_entities_created_at ON example_entities (created_at DESC);

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger to call the function before any UPDATE
CREATE TRIGGER update_example_entities_updated_at
    BEFORE UPDATE ON example_entities
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();