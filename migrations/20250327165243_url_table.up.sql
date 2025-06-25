-- Add up migration script here
CREATE TABLE urls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    original_url VARCHAR(225) NOT NULL UNIQUE,
    short_code VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
