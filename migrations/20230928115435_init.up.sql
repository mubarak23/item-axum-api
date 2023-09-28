-- Add up migration script here
-- to run the migration
-- run this command 
---  sqlx migrate run --database-url postgres://postgres:5jrfjnrhgbrh@localhost/livecode
-- to revert the migration,
-- run 
--- sqlx migrate revert --database-url postgres://postgres:3fhrfyu5t5y@localhost/livecode

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS notes (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        title VARCHAR(255) NOT NULL UNIQUE,
        content TEXT NOT NULL,
        category VARCHAR(100),
        published BOOLEAN DEFAULT FALSE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
