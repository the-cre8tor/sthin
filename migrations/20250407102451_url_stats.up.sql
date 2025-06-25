-- Add up migration script here
CREATE TABLE url_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    url_id UUID NOT NULL UNIQUE,
    access_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (url_id) REFERENCES urls (id) ON DELETE CASCADE
)
