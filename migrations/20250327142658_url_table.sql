-- Add migration script here
CREATE TABLE urls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    original_url TEXT NOT NULL,
    short_code VARCHAR(10) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT now (),
    updated_at TIMESTAMP DEFAULT now ()
);
