-- Add migration script here
TRUNCATE TABLE todo_items;

ALTER TABLE todo_items
ADD COLUMN IF NOT EXISTS user_id UUID REFERENCES users (id) ON DELETE CASCADE;
-- Add an index on the user_id column for performance
CREATE INDEX IF NOT EXISTS idx_todo_items_user_id ON todo_items (user_id);