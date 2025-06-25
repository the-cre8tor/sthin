-- Add up migration script here
BEGIN;

CREATE TABLE url_stats_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    url_stats_id UUID NOT NULL,
    ip_address VARCHAR(40) NOT NULL,
    user_agent TEXT NOT NULL,
    accessed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (url_stats_id) REFERENCES url_stats (id) ON DELETE CASCADE
);

CREATE INDEX idx_url_stats_logs_url_stats_id ON url_stats_logs(url_stats_id);

COMMIT;
