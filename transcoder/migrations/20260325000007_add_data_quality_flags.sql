CREATE TABLE IF NOT EXISTS data_quality_flags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    watch_history_id UUID REFERENCES watch_history(id) ON DELETE CASCADE,
    flag_type VARCHAR NOT NULL,
    flag_severity VARCHAR NOT NULL,
    description TEXT,
    resolved BOOLEAN DEFAULT FALSE,
    resolved_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_quality_flags_resolved 
  ON data_quality_flags(resolved, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_quality_flags_severity 
  ON data_quality_flags(flag_severity);
